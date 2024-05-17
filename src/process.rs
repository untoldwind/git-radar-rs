use std::{
    error::Error,
    process::{Command, ExitStatus, Stdio},
    str,
};

pub fn process_with_exit_code(
    command: &str,
    options: &[&str],
) -> Result<(ExitStatus, Vec<u8>), Box<dyn Error>> {
    let output = Command::new(command)
        .args(options)
        .stdout(Stdio::piped())
        .output()?;

    Ok((output.status, output.stdout))
}

pub fn process_with_ignore_exit_code(
    command: &str,
    options: &[&str],
) -> Result<Vec<u8>, Box<dyn Error>> {
    let (status, stdout) = process_with_exit_code(command, options)?;

    if status.success() {
        Ok(stdout)
    } else {
        Ok(vec![])
    }
}
