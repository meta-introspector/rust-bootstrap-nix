use anyhow::{Context, Result, ensure};
use std::path::{Path, PathBuf};
use indoc::indoc;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct GitProjectProvisioner;

impl GitProjectProvisioner {
    pub async fn provision_project(
        source_code: &str,
        original_file_path: &Path,
        generated_output_dir: &Path,
    ) -> Result<PathBuf> {
        // Generate a short, unique ID for the project directory
        let mut hasher = DefaultHasher::new();
        original_file_path.hash(&mut hasher);
        let short_id = format!("{:x}", hasher.finish());

        let hf_validator_project_dir = generated_output_dir.join(format!("hf_validator_projects/{}", short_id));
        tokio::fs::create_dir_all(&hf_validator_project_dir).await?;

        // Write the Rust source code to a file within the persistent directory
        let source_file_path = hf_validator_project_dir.join("main.rs");
        tokio::fs::write(&source_file_path, source_code.as_bytes()).await
            .context("Failed to write source code to persistent file")?;

        // Create a minimal Cargo.toml in the persistent directory for hf-validator
        let cargo_toml_content = indoc! {
            r#"[package]
            name = "temp_hf_project"
            version = "0.1.0"
            edition = "2021"

            [[bin]]
            name = "temp_hf_project"
            path = "main.rs"

            [dependencies]
            anyhow = "1.0"
            tokio = { version = "1", features = ["full"] }

            [workspace]
            "#
        };
        tokio::fs::write(hf_validator_project_dir.join("Cargo.toml"), cargo_toml_content).await?;

        // Initialize a Git repository and make a dummy commit
        tokio::process::Command::new("git")
            .arg("init")
            .current_dir(&hf_validator_project_dir)
            .output().await
            .context("Failed to initialize git repository")
            .and_then(|output| {
                anyhow::ensure!(output.status.success(), "git init failed: {}", String::from_utf8_lossy(&output.stderr));
                Ok(())
            })?;

        // Configure dummy Git user and email
        tokio::process::Command::new("git")
            .arg("config")
            .arg("user.email")
            .arg("test@example.com")
            .current_dir(&hf_validator_project_dir)
            .output().await
            .context("Failed to configure git user email")
            .and_then(|output| {
                anyhow::ensure!(output.status.success(), "git config user.email failed: {}", String::from_utf8_lossy(&output.stderr));
                Ok(())
            })?;

        tokio::process::Command::new("git")
            .arg("config")
            .arg("user.name")
            .arg("Test User")
            .current_dir(&hf_validator_project_dir)
            .output().await
            .context("Failed to configure git user name")
            .and_then(|output| {
                anyhow::ensure!(output.status.success(), "git config user.name failed: {}", String::from_utf8_lossy(&output.stderr));
                Ok(())
            })?;

        // Create an initial empty commit to satisfy hf-validator's git requirements
        tokio::process::Command::new("git")
            .arg("commit")
            .arg("--allow-empty")
            .arg("-m")
            .arg("Initial empty commit")
            .current_dir(&hf_validator_project_dir)
            .output().await
            .context("Failed to make initial empty git commit")
            .and_then(|output| {
                anyhow::ensure!(output.status.success(), "git commit --allow-empty failed: {}", String::from_utf8_lossy(&output.stderr));
                Ok(())
            })?;

        Ok(hf_validator_project_dir)
    }
}
