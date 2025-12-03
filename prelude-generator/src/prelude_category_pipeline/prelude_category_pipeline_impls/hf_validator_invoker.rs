use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tempfile::tempdir;
use crate::args::Args; // Assuming Args is needed for self.args.path

pub struct HfValidatorInvoker {
    pub hf_validator_path: Option<PathBuf>,
    pub args: Args, // Store Args to access self.args.path
}

impl HfValidatorInvoker {
    pub async fn invoke_validator(
        &self,
        hf_validator_project_dir: &Path,
        writer: &mut (impl tokio::io::AsyncWriteExt + Unpin + Send),
    ) -> Result<PathBuf> {
        let temp_output_dir = tempdir()
            .context("Failed to create temporary output directory for hf-validator")?;
        let output_path = temp_output_dir.path().to_path_buf();

        let hf_validator_executable = self.hf_validator_path.clone().unwrap_or_else(|| {
            // Fallback to default if not provided in config.toml, assuming release build
            PathBuf::from(&self.args.path).join("target/release/hf-validator")
        });

        writer.write_all(format!("  -> Executing hf-validator: {:#?}\n", hf_validator_executable).as_bytes()).await?;
        if let Some(path_env) = std::env::var_os("PATH") {
            writer.write_all(format!("  -> PATH: {:#?}\n", path_env).as_bytes()).await?;
        }
        if let Some(ld_library_path_env) = std::env::var_os("LD_LIBRARY_PATH") {
            writer.write_all(format!("  -> LD_LIBRARY_PATH: {:#?}\n", ld_library_path_env).as_bytes()).await?;
        }

        let status = tokio::process::Command::new(hf_validator_executable.to_str().unwrap())
            .current_dir(&self.args.path) // Set current_dir to project root
            .envs(std::env::vars_os()) // Pass all current environment variables
            .arg("analyze-rust-to-ir")
            .arg(hf_validator_project_dir.as_os_str())
            .arg(output_path.as_os_str())
            .status().await
            .context("Failed to execute hf-validator command")?;

        if !status.success() {
            return Err(anyhow::anyhow!("hf-validator command failed with status: {}", status));
        }

        writer.write_all(format!("  -> Hugging Face Validation Result: Dataset generated at {:#?}\n", output_path).as_bytes()).await?;

        Ok(output_path)
    }
}
