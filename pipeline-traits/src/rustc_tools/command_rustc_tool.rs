use anyhow::{Result, anyhow};
use async_trait::async_trait;
use std::path::PathBuf;
use std::process::Command;

use crate::rustc_tool_trait::RustcToolTrait;

pub struct CommandRustcTool {
    pub rustc_path: PathBuf,
}

#[async_trait]
impl RustcToolTrait for CommandRustcTool {
    async fn compile(&self, input_path: &PathBuf, output_path: &PathBuf) -> Result<()> {
        let output = Command::new(&self.rustc_path)
            .arg(input_path)
            .arg("-o")
            .arg(output_path)
            .output()
            .map_err(|e| anyhow!("Failed to execute rustc compile command: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(anyhow!(
                "rustc compile failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    async fn check(&self, input_path: &PathBuf) -> Result<()> {
        let output = Command::new(&self.rustc_path)
            .arg(input_path)
            .arg("--crate-type=lib") // Assuming checking a library
            .arg("--emit=dep-info") // Emit dependency info without compiling
            .output()
            .map_err(|e| anyhow!("Failed to execute rustc check command: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(anyhow!(
                "rustc check failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    async fn version(&self) -> Result<String> {
        let output = Command::new(&self.rustc_path)
            .arg("--version")
            .output()
            .map_err(|e| anyhow!("Failed to execute rustc version command: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(anyhow!(
                "rustc version command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    async fn run_command(&self, args: &[&str]) -> Result<String> {
        let output = Command::new(&self.rustc_path)
            .args(args)
            .output()
            .map_err(|e| anyhow!("Failed to execute rustc command: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(anyhow!(
                "rustc command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }
}
