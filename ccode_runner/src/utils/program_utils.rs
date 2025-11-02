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
/// - **Time limits**: Supported on all platforms (Unix, Windows, macOS)
/// - **Memory limits**: Supported on all platforms
///   - Unix/Linux/macOS: Uses `setrlimit(RLIMIT_AS)` for native OS enforcement
///   - Windows: Uses active monitoring via `sysinfo` to track and enforce limits
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
    /// - Unix/Linux/macOS: Native OS enforcement via `setrlimit`
    /// - Windows: Active monitoring and enforcement via process memory tracking
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

    // Apply memory limit on Unix systems
    #[cfg(unix)]
    if let Some(memory_limit) = limits.memory_limit_bytes {
        apply_memory_limit(&mut command, memory_limit);
    }

    let mut child = command.spawn()?;

    // Monitor memory on Windows (Unix has native enforcement via setrlimit)
    #[cfg(not(unix))]
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
                #[cfg(not(unix))]
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
                #[cfg(not(unix))]
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

        #[cfg(not(unix))]
        if let Some(monitor) = memory_monitor {
            stop_memory_monitor(monitor);
        }

        output
    };

    run_program_common(output, program, args)
}

#[cfg(unix)]
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

// Windows memory monitoring using sysinfo
// Similar to the approach used in codemark-cli
#[cfg(not(unix))]
struct MemoryMonitor {
    should_stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    handle: Option<std::thread::JoinHandle<()>>,
}

#[cfg(not(unix))]
fn start_memory_monitor(pid: u32, memory_limit_bytes: u64) -> MemoryMonitor {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use sysinfo::{Pid, System, Signal, ProcessesToUpdate};

    let should_stop = Arc::new(AtomicBool::new(false));
    let should_stop_clone = should_stop.clone();

    let handle = thread::spawn(move || {
        let mut sys = System::new();
        let pid = Pid::from_u32(pid);

        while !should_stop_clone.load(Ordering::Relaxed) {
            sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

            if let Some(process) = sys.process(pid) {
                let memory_usage = process.memory();

                if memory_usage > memory_limit_bytes {
                    // Try to kill the process
                    if process.kill() || process.kill_with(Signal::Kill).unwrap_or(false) {
                        eprintln!("Process terminated due to exceeding memory limit.");
                    }
                    break;
                }
            } else {
                // Process has exited
                break;
            }

            // Check memory every 100ms
            thread::sleep(Duration::from_millis(100));
        }
    });

    MemoryMonitor {
        should_stop,
        handle: Some(handle),
    }
}

#[cfg(not(unix))]
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

    // Apply memory limit on Unix systems
    #[cfg(unix)]
    if let Some(memory_limit) = limits.memory_limit_bytes {
        apply_memory_limit(&mut command, memory_limit);
    }

    let mut child = command.spawn()?;

    // Monitor memory on Windows (Unix has native enforcement via setrlimit)
    #[cfg(not(unix))]
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
                #[cfg(not(unix))]
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
                #[cfg(not(unix))]
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

        #[cfg(not(unix))]
        if let Some(monitor) = memory_monitor {
            stop_memory_monitor(monitor);
        }

        output
    };

    run_program_common(output, program, args)
}
