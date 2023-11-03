use std::process::{Command, Stdio};
use std::{io, io::Write};
use which::which;

fn program_exists(program: &str) -> Result<std::path::PathBuf, which::Error> {
    which(program)
}

fn run_program(program: &str, args: Vec<&str>, stdin_content: String) -> io::Result<String> {
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        // Close stdin to finish and avoid indefinite blocking
        stdin.write_all(stdin_content.as_ref())?; // drop would happen here
    }

    let output = child.wait_with_output()?;

    let stdout_content = String::from_utf8(output.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .expect("Found invalid UTF-8");

    Ok(stdout_content)
}
