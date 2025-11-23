use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;

#[async_trait]
pub trait RustcToolTrait: Send + Sync {
    /// Compiles a Rust project or file.
    async fn compile(&self, input_path: &PathBuf, output_path: &PathBuf) -> Result<()>;

    /// Checks a Rust project or file for errors without producing an executable.
    async fn check(&self, input_path: &PathBuf) -> Result<()>;

    /// Returns the version of the rustc tool.
    async fn version(&self) -> Result<String>;

    /// Runs a custom rustc command with provided arguments.
    async fn run_command(&self, args: &[&str]) -> Result<String>;
}
