use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;
use std::process::Command;
use crate::external_interfaces::GitInterface;

pub struct GitInterfaceImpl;

impl GitInterface for GitInterfaceImpl {
    fn run_command(&self, args: &[&str], current_dir: Option<&PathBuf>) -> Result<String> {
        let mut command = Command::new("git");
        command.args(args);
        if let Some(dir) = current_dir {
            command.current_dir(dir);
        }
        let output = command.output().context("Failed to execute git command")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(anyhow!("Git command failed: {}\nStdout: {}\nStderr: {}",
                args.join(" "),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    fn get_current_branch(&self, current_dir: Option<&PathBuf>) -> Result<String> {
        self.run_command(&["rev-parse", "--abbrev-ref", "HEAD"], current_dir)
    }

    fn get_last_commit_hash(&self, current_dir: Option<&PathBuf>) -> Result<String> {
        self.run_command(&["rev-parse", "HEAD"], current_dir)
    }
}

