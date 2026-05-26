use std::path::Path;
use std::process::Command;
use std::process::{Output, Stdio};
use std::time::Duration;
use std::{io, io::Write};
use wait_timeout::ChildExt;
use which::which;

/// Execution limits for running programs
///
/// # Platform Support
///
/// - **Time limits**: Supported on all platforms (Linux, macOS, Windows)
/// - **Memory limits**: Supported on all platforms
///   - Linux: Uses `setrlimit(RLIMIT_AS)` for native OS enforcement
///   - macOS/Windows: Uses active RSS monitoring via `sysinfo` to track and kill over-limit processes
///     (`RLIMIT_AS` is not used on macOS because high-level runtimes like Python pre-map large
///     virtual address spaces at startup, making virtual-memory limits unreliable)
#[derive(Debug, Clone, Copy, Default)]
pub struct ExecutionLimits {
    /// Time limit in milliseconds (None means no limit)
    pub time_limit_ms: Option<u64>,
    /// Memory limit in bytes (None means no limit)
    pub memory_limit_bytes: Option<u64>,
}

impl ExecutionLimits {
    /// Create new execution limits with no restrictions
    pub fn new() -> Self {
        Self::default()
    }

    /// Set time limit in milliseconds
    pub fn with_time_limit(mut self, time_limit_ms: u64) -> Self {
        self.time_limit_ms = Some(time_limit_ms);
        self
    }

    /// Set memory limit in bytes
    ///
    /// Memory limits are enforced on all platforms:
    /// - Linux: Native OS enforcement via `setrlimit(RLIMIT_AS)`
    /// - macOS/Windows: Active RSS monitoring via `sysinfo` — polls every 100ms and kills the
    ///   process if it exceeds the limit
    pub fn with_memory_limit(mut self, memory_limit_bytes: u64) -> Self {
        self.memory_limit_bytes = Some(memory_limit_bytes);
        self
    }
}

fn program_exists(program: &str) -> Result<std::path::PathBuf, which::Error> {
    which(program)
}

fn run_program_common(output: Output, program: &str, args: &[&str]) -> io::Result<String> {
    if output.status.code() != Some(0) {
        return Err(io::Error::other(format!(
            "Process `{} {}` failed to run successfully!\nStatus Code: {}\nOutput: {}\nError: {}",
            program,
            args.join(" "),
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let stdout_content = String::from_utf8(output.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .expect("Found invalid UTF-8");

    Ok(stdout_content)
}

pub(crate) fn run_program_with_input(
    program: &str,
    args: &Vec<&str>,
    stdin_content: &str,
    limits: &ExecutionLimits,
) -> io::Result<String> {
    if let Err(err) = program_exists(program) {
        return Err(io::Error::other(err));
    }

    let mut command = Command::new(program);
    command
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Apply memory limit via setrlimit on Linux only.
    // macOS is excluded: RLIMIT_AS limits virtual address space, but runtimes like Python
    // pre-map large virtual ranges at startup, making this limit unreliable on macOS.
    #[cfg(target_os = "linux")]
    if let Some(memory_limit) = limits.memory_limit_bytes {
        apply_memory_limit(&mut command, memory_limit);
    }

    let mut child = command.spawn()?;

    // Active memory monitor for macOS and Windows (Linux uses native setrlimit instead)
    #[cfg(not(target_os = "linux"))]
    let memory_monitor = if let Some(memory_limit) = limits.memory_limit_bytes {
        Some(start_memory_monitor(child.id(), memory_limit))
    } else {
        None
    };

    if let Some(mut stdin) = child.stdin.take() {
        // Close stdin to finish and avoid indefinite blocking
        stdin.write_all(stdin_content.as_ref())?; // drop would happen here
    }

    // Handle timeout if specified
    let output = if let Some(time_limit_ms) = limits.time_limit_ms {
        let timeout = Duration::from_millis(time_limit_ms);
        match child.wait_timeout(timeout)? {
            Some(status) => {
                // Process exited before timeout
                #[cfg(not(target_os = "linux"))]
                if let Some(monitor) = memory_monitor {
                    stop_memory_monitor(monitor);
                }

                let stdout = child
                    .stdout
                    .take()
                    .map(|mut s| {
                        let mut buf = Vec::new();
                        let _ = std::io::Read::read_to_end(&mut s, &mut buf);
                        buf
                    })
                    .unwrap_or_default();

                let stderr = child
                    .stderr
                    .take()
                    .map(|mut s| {
                        let mut buf = Vec::new();
                        let _ = std::io::Read::read_to_end(&mut s, &mut buf);
                        buf
                    })
                    .unwrap_or_default();

                Output {
                    status,
                    stdout,
                    stderr,
                }
            }
            None => {
                // Timeout occurred, kill the process
                #[cfg(not(target_os = "linux"))]
                if let Some(monitor) = memory_monitor {
                    stop_memory_monitor(monitor);
                }

                let _ = child.kill();
                return Err(io::Error::other(format!(
                    "Process `{} {}` exceeded time limit of {} ms",
                    program,
                    args.join(" "),
                    time_limit_ms
                )));
            }
        }
    } else {
        let output = child.wait_with_output()?;

        #[cfg(not(target_os = "linux"))]
        if let Some(monitor) = memory_monitor {
            stop_memory_monitor(monitor);
        }

        output
    };

    run_program_common(output, program, args)
}

#[cfg(target_os = "linux")]
fn apply_memory_limit(command: &mut Command, memory_limit_bytes: u64) {
    use std::os::unix::process::CommandExt;

    unsafe {
        command.pre_exec(move || {
            // RLIMIT_AS limits the virtual memory
            let limit = libc::rlimit {
                rlim_cur: memory_limit_bytes,
                rlim_max: memory_limit_bytes,
            };

            if libc::setrlimit(libc::RLIMIT_AS, &limit) != 0 {
                eprintln!("Warning: Failed to set memory limit");
            }

            Ok(())
        });
    }
}

// Active memory monitoring for macOS and Windows.
// Linux uses setrlimit(RLIMIT_AS) instead (see apply_memory_limit).
//
// Platform metrics used:
//   - macOS: `phys_footprint` via proc_pid_rusage(RUSAGE_INFO_V2)
//            This is the same metric Activity Monitor and Xcode use.
//            It includes resident memory + compressed memory + IOKit memory,
//            making it far more accurate than RSS on macOS.
//   - Windows: RSS via sysinfo (best available cross-platform approximation)
#[cfg(not(target_os = "linux"))]
struct MemoryMonitor {
    should_stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    handle: Option<std::thread::JoinHandle<()>>,
}

/// Reads the physical memory footprint of a process on macOS using
/// `proc_pid_rusage(pid, RUSAGE_INFO_V2, ...)` from `<libproc.h>`.
///
/// `phys_footprint` is the canonical macOS memory metric — it accounts for:
/// - Resident pages (in RAM)
/// - Compressed pages (held in the memory compressor)
/// - IOKit/GPU memory
///
/// Unlike RSS, it does not double-count shared libraries and correctly
/// captures memory that the OS has compressed rather than paged out.
#[cfg(target_os = "macos")]
fn get_phys_footprint(pid: u32) -> Option<u64> {
    // rusage_info_v2 layout from <sys/resource.h>
    // We only need phys_footprint (field at offset 72 bytes),
    // but we must provide the full struct for the syscall.
    #[repr(C)]
    struct RusageInfoV2 {
        ri_uuid: [u8; 16],
        ri_user_time: u64,
        ri_system_time: u64,
        ri_pkg_idle_wkups: u64,
        ri_interrupt_wkups: u64,
        ri_pageins: u64,
        ri_wired_size: u64,
        ri_resident_size: u64,
        ri_phys_footprint: u64,
        ri_proc_start_abstime: u64,
        ri_proc_exit_abstime: u64,
        ri_child_user_time: u64,
        ri_child_system_time: u64,
        ri_child_pkg_idle_wkups: u64,
        ri_child_interrupt_wkups: u64,
        ri_child_pageins: u64,
        ri_child_elapsed_abstime: u64,
        ri_diskio_bytesread: u64,
        ri_diskio_byteswritten: u64,
        ri_cpu_time_qos_default: u64,
        ri_cpu_time_qos_maintenance: u64,
        ri_cpu_time_qos_background: u64,
        ri_cpu_time_qos_utility: u64,
        ri_cpu_time_qos_legacy: u64,
        ri_cpu_time_qos_user_initiated: u64,
        ri_cpu_time_qos_user_interactive: u64,
        ri_billed_system_time: u64,
        ri_serviced_system_time: u64,
    }

    // RUSAGE_INFO_V2 = 2 (from <sys/resource.h>)
    const RUSAGE_INFO_V2: libc::c_int = 2;

    let mut info = std::mem::MaybeUninit::<RusageInfoV2>::uninit();

    let ret = unsafe {
        // int proc_pid_rusage(int pid, int flavor, rusage_info_t *buffer, int *error)
        // Declared in <libproc.h>, linked via libproc (part of libSystem on macOS).
        unsafe extern "C" {
            fn proc_pid_rusage(
                pid: libc::c_int,
                flavor: libc::c_int,
                buffer: *mut libc::c_void,
                error: *mut libc::c_int,
            ) -> libc::c_int;
        }

        let mut err: libc::c_int = 0;
        proc_pid_rusage(
            pid as libc::c_int,
            RUSAGE_INFO_V2,
            info.as_mut_ptr() as *mut libc::c_void,
            &mut err,
        )
    };

    if ret == 0 {
        let info = unsafe { info.assume_init() };
        Some(info.ri_phys_footprint)
    } else {
        None
    }
}

#[cfg(not(target_os = "linux"))]
fn start_memory_monitor(pid: u32, memory_limit_bytes: u64) -> MemoryMonitor {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;

    let should_stop = Arc::new(AtomicBool::new(false));
    let should_stop_clone = should_stop.clone();

    let handle = thread::spawn(move || {
        #[cfg(not(target_os = "macos"))]
        let mut sys = sysinfo::System::new();

        while !should_stop_clone.load(Ordering::Relaxed) {
            let memory_usage = {
                #[cfg(target_os = "macos")]
                {
                    get_phys_footprint(pid)
                }

                #[cfg(not(target_os = "macos"))]
                {
                    use sysinfo::{Pid, ProcessesToUpdate};
                    let pid_key = Pid::from_u32(pid);
                    sys.refresh_processes(ProcessesToUpdate::Some(&[pid_key]), true);
                    sys.process(pid_key).map(|p| p.memory())
                }
            };

            match memory_usage {
                Some(usage) if usage > memory_limit_bytes => {
                    // Kill the process — platform-specific:
                    // - macOS/Unix: libc::kill with SIGKILL (libc is a unix-only dep)
                    // - Windows: sysinfo process lookup + kill() (libc not available)
                    #[cfg(unix)]
                    unsafe {
                        libc::kill(pid as libc::pid_t, libc::SIGKILL);
                    }

                    #[cfg(windows)]
                    {
                        use sysinfo::{Pid, ProcessesToUpdate};
                        let pid_key = Pid::from_u32(pid);
                        sys.refresh_processes(ProcessesToUpdate::Some(&[pid_key]), true);
                        if let Some(proc) = sys.process(pid_key) {
                            let _ = proc.kill();
                        }
                    }

                    eprintln!(
                        "Process {} terminated: memory usage {} bytes exceeded limit of {} bytes",
                        pid, usage, memory_limit_bytes
                    );
                    break;
                }
                None => {
                    // Process has already exited
                    break;
                }
                _ => {}
            }

            // Poll every 5ms — balances CPU overhead vs. overshoot window
            thread::sleep(Duration::from_millis(5));
        }
    });

    MemoryMonitor {
        should_stop,
        handle: Some(handle),
    }
}

#[cfg(not(target_os = "linux"))]
fn stop_memory_monitor(mut monitor: MemoryMonitor) {
    use std::sync::atomic::Ordering;

    monitor.should_stop.store(true, Ordering::Relaxed);

    if let Some(handle) = monitor.handle.take() {
        let _ = handle.join();
    }
}

/// Adapted with modifications from GNU Make Project
/// * `source_code_path` : Path of source code
/// * `compiled_artifact_path` : The name of compiled artifact, generally file-stem name of `source_code_path`
///   Returns true if file needs to be recompiled
pub(crate) fn remake(
    source_code_path: &Path,
    compiled_artifact_path: &Path,
) -> Result<bool, io::Error> {
    if compiled_artifact_path.exists() {
        let source_modified_time = source_code_path.metadata()?.modified()?;
        let compiled_artifact_creation_time = compiled_artifact_path.metadata()?.created()?;

        return Ok(source_modified_time > compiled_artifact_creation_time);
    }
    Ok(true)
}

pub(crate) fn run_program(
    program: &str,
    args: &Vec<&str>,
    limits: &ExecutionLimits,
) -> io::Result<String> {
    if let Err(err) = program_exists(program) {
        return Err(io::Error::other(err));
    }

    let mut command = Command::new(program);
    command
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Apply memory limit via setrlimit on Linux only (see run_program_with_input for rationale)
    #[cfg(target_os = "linux")]
    if let Some(memory_limit) = limits.memory_limit_bytes {
        apply_memory_limit(&mut command, memory_limit);
    }

    let mut child = command.spawn()?;

    // Active memory monitor for macOS and Windows
    #[cfg(not(target_os = "linux"))]
    let memory_monitor = if let Some(memory_limit) = limits.memory_limit_bytes {
        Some(start_memory_monitor(child.id(), memory_limit))
    } else {
        None
    };

    // Handle timeout if specified
    let output = if let Some(time_limit_ms) = limits.time_limit_ms {
        let timeout = Duration::from_millis(time_limit_ms);
        match child.wait_timeout(timeout)? {
            Some(status) => {
                // Process exited before timeout
                #[cfg(not(target_os = "linux"))]
                if let Some(monitor) = memory_monitor {
                    stop_memory_monitor(monitor);
                }

                let stdout = child
                    .stdout
                    .take()
                    .map(|mut s| {
                        let mut buf = Vec::new();
                        let _ = std::io::Read::read_to_end(&mut s, &mut buf);
                        buf
                    })
                    .unwrap_or_default();

                let stderr = child
                    .stderr
                    .take()
                    .map(|mut s| {
                        let mut buf = Vec::new();
                        let _ = std::io::Read::read_to_end(&mut s, &mut buf);
                        buf
                    })
                    .unwrap_or_default();

                Output {
                    status,
                    stdout,
                    stderr,
                }
            }
            None => {
                // Timeout occurred, kill the process
                #[cfg(not(target_os = "linux"))]
                if let Some(monitor) = memory_monitor {
                    stop_memory_monitor(monitor);
                }

                let _ = child.kill();
                return Err(io::Error::other(format!(
                    "Process `{} {}` exceeded time limit of {} ms",
                    program,
                    args.join(" "),
                    time_limit_ms
                )));
            }
        }
    } else {
        let output = child.wait_with_output()?;

        #[cfg(not(target_os = "linux"))]
        if let Some(monitor) = memory_monitor {
            stop_memory_monitor(monitor);
        }

        output
    };

    run_program_common(output, program, args)
}
