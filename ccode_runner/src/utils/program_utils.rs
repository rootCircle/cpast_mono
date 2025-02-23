use std::io;
use std::path::Path;
use std::process::{Output, Stdio};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use which::which;

fn program_exists(program: &str) -> Result<std::path::PathBuf, which::Error> {
    which(program)
}

fn run_program_common(output: Output, program: &str, args: &[&str]) -> io::Result<String> {
    if output.status.code() != Some(0) {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Process `{} {}` failed to run successfully!\nStatus Code: {}\nOutput: {}\nError: {}",
                program,
                args.join(" "),
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    let stdout_content = String::from_utf8(output.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .expect("Found invalid UTF-8");

    Ok(stdout_content)
}

pub(crate) async fn run_program_with_input(
    program: &str,
    args: &Vec<&str>,
    stdin_content: &str,
) -> io::Result<String> {
    if let Err(err) = program_exists(program) {
        return Err(io::Error::new(io::ErrorKind::Other, err));
    }

    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        // Close stdin to finish and avoid indefinite blocking
        stdin.write_all(stdin_content.as_ref()).await?; // drop would happen here
    }

    let output = child.wait_with_output().await?;

    run_program_common(output, program, args)
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

pub(crate) async fn run_program(program: &str, args: &Vec<&str>) -> io::Result<String> {
    if let Err(err) = program_exists(program) {
        return Err(io::Error::new(io::ErrorKind::Other, err));
    }

    let child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    let output = match child.await {
        Ok(out) => out,
        Err(err) => {
            eprintln!(
                "[PROGRAM UTILS ERROR] Failed to run the command {} {}",
                program,
                args.join(" ")
            );
            return Err(io::Error::new(io::ErrorKind::Other, err));
        }
    };

    run_program_common(output, program, args)
}
