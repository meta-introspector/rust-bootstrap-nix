use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;
use crate::external_interfaces::IoInterface;

pub struct IoInterfaceImpl;

impl IoInterface for IoInterfaceImpl {
    fn read_file(&self, path: &PathBuf) -> impl std::future::Future<Output = Result<String>> + Send {
        async move {
            tokio::fs::read_to_string(path).await
                .with_context(|| format!("Failed to read file: {}", path.display()))
        }
    }

    fn write_file(&self, path: &PathBuf, content: &str) -> impl std::future::Future<Output = Result<()>> + Send {
        async move {
            tokio::fs::write(path, content).await
                .with_context(|| format!("Failed to write file: {}", path.display()))
        }
    }

    fn create_dir_all(&self, path: &PathBuf) -> impl std::future::Future<Output = Result<()>> + Send {
        async move {
            tokio::fs::create_dir_all(path).await
                .with_context(|| format!("Failed to create directory: {}", path.display()))
        }
    }

    fn remove_dir_all(&self, path: &PathBuf) -> impl std::future::Future<Output = Result<()>> + Send {
        async move {
            tokio::fs::remove_dir_all(path).await
                .with_context(|| format!("Failed to remove directory: {}", path.display()))
        }
    }

    fn path_exists(&self, path: &PathBuf) -> impl std::future::Future<Output = bool> + Send {
        async move {
            path.exists()
        }
    }

    fn run_command(&self, program: &str, args: &[&str], current_dir: Option<&PathBuf>) -> impl std::future::Future<Output = Result<String>> + Send {
        async move {
            let mut command = tokio::process::Command::new(program);
            command.args(args);
            if let Some(dir) = current_dir {
                command.current_dir(dir);
            }
            let output = command.output().await.with_context(|| format!("Failed to execute command: {}", program))?;

            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                Err(anyhow!("Command failed: {}\nStdout: {}\nStderr: {}",
                    program,
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ))
            }
        }
    }
}
