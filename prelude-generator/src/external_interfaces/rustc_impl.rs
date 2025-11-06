use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;
use std::process::Command;
use crate::external_interfaces::RustcInterface;

pub struct RustcInterfaceImpl;

impl RustcInterface for RustcInterfaceImpl {
    fn run_command(&self, args: &[&str], current_dir: Option<&PathBuf>) -> Result<String> {
        let mut command = Command::new("rustc");
        command.args(args);
        if let Some(dir) = current_dir {
            command.current_dir(dir);
        }
        let output = command.output().context("Failed to execute rustc command")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(anyhow!("Rustc command failed: {}\nStdout: {}\nStderr: {}",
                args.join(" "),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    fn get_version_verbose(&self) -> Result<String> {
        self.run_command(&["--version", "--verbose"], None)
    }

    fn get_sysroot(&self) -> Result<PathBuf> {
        let output = self.run_command(&["--print", "sysroot"], None)?;
        Ok(PathBuf::from(output.trim()))
    }
}
