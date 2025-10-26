


use std::path::Path;
use std::process::{Command, Stdio};
use anyhow::{Context, Result};

/// Runs a git command and returns the output, handling errors.
pub fn run_git_command(
    current_dir: &Path,
    args: &[&str],
    error_message: &str,
    dry_run: bool,
) -> Result<()> {
    println!("Running git command in CWD: {:?}", current_dir);
    let command_str = format!("git {}", args.join(" "));
    if dry_run {
        println!("Dry run: Would execute: {}", command_str);
        return Ok(())
    }
    println!("Executing: {}", command_str);

    let output = Command::new("git")
        .current_dir(current_dir)
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute git command: {}", command_str))?;

    if !output.status.success() {
        anyhow::bail!(
            "Git command failed: {}
Stdout: {}
Stderr: {}",
            error_message,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

/// Runs a command and returns the stdout as a String.
pub fn output_result(cmd: &mut Command) -> Result<String> {
    let output = cmd.stderr(Stdio::inherit()).output()
        .with_context(|| format!("Failed to run command: {:?}", cmd))?;

    if !output.status.success() {
        anyhow::bail!(
            "Command did not execute successfully: {:?}\nExpected success, got: {}
Stderr: {}",
            cmd,
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    String::from_utf8(output.stdout)
        .with_context(|| "Failed to convert stdout to UTF-8".to_string())
}
