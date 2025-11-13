use anyhow::{Context, Result};
use std::path::PathBuf;
use std::pin::Pin;
use std::future::Future;
use std::boxed::Box;

use crate::measurement;
use pipeline_traits::{PipelineFunctor, ParsedFile, ValidatedFile};
use indoc::indoc;
use tempfile::tempdir;
use super::utils::copy_dir_all;

// HuggingFaceValidatorFunctor
pub struct HuggingFaceValidatorFunctor {
    pub args: crate::Args,
    pub hf_validator_path: Option<PathBuf>,
}

impl PipelineFunctor<ParsedFile, ValidatedFile> for HuggingFaceValidatorFunctor {
    fn map<'writer>(
        &'writer self,
        writer: &'writer mut (impl tokio::io::AsyncWriteExt + Unpin + Send),
        input: ParsedFile,
    ) -> Pin<Box<dyn Future<Output = Result<ValidatedFile>> + Send + 'writer>> {
        Box::pin(async move {
            measurement::record_function_entry("HuggingFaceValidatorFunctor::map");
            let ParsedFile(source_code, original_file_path) = input;

            // The source_code is already a String, no need to unparse from AST
            // Generate a short, unique ID for the project directory to avoid "file name too long" errors.
            use std::hash::{Hash, Hasher};
            use std::collections::hash_map::DefaultHasher;
            let mut hasher = DefaultHasher::new();
            original_file_path.hash(&mut hasher);
            let short_id = format!("{:x}", hasher.finish());

            writer.write_all(format!("  -> Short ID for hf-validator project: {}\n", short_id).as_bytes()).await?;
            let hf_validator_project_dir = PathBuf::from(format!("generated/hf_validator_projects/{}", short_id));
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
            let output = tokio::process::Command::new("git")
                .arg("init")
                .current_dir(&hf_validator_project_dir)
                .output().await
                .context("Failed to initialize git repository")?;
            if !output.status.success() {
                return Err(anyhow::anyhow!("git init failed: {}", String::from_utf8_lossy(&output.stderr)));
            }

            // Configure dummy Git user and email
            let output = tokio::process::Command::new("git")
                .arg("config")
                .arg("user.email")
                .arg("test@example.com")
                .current_dir(&hf_validator_project_dir)
                .output().await
                .context("Failed to configure git user email")?;
            if !output.status.success() {
                return Err(anyhow::anyhow!("git config user.email failed: {}\nStderr: {}", String::from_utf8_lossy(&output.status.code().unwrap_or(-1).to_string().as_bytes()), String::from_utf8_lossy(&output.stderr)));
            }

            let output = tokio::process::Command::new("git")
                .arg("config")
                .arg("user.name")
                .arg("Test User")
                .current_dir(&hf_validator_project_dir)
                .output().await
                .context("Failed to configure git user name")?;
            if !output.status.success() {
                return Err(anyhow::anyhow!("git config user.name failed: {}\nStderr: {}", String::from_utf8_lossy(&output.status.code().unwrap_or(-1).to_string().as_bytes()), String::from_utf8_lossy(&output.stderr)));
            }

            // Create an initial empty commit to satisfy hf-validator's git requirements
            let output = tokio::process::Command::new("git")
                .arg("commit")
                .arg("--allow-empty")
                .arg("-m")
                .arg("Initial empty commit")
                .current_dir(&hf_validator_project_dir)
                .output().await
                .context("Failed to make initial empty git commit")?;
            if !output.status.success() {
                return Err(anyhow::anyhow!("git commit --allow-empty failed: {}\nStderr: {}", String::from_utf8_lossy(&output.status.code().unwrap_or(-1).to_string().as_bytes()), String::from_utf8_lossy(&output.stderr)));
            }

            // Create a temporary directory for the output of hf-dataset-validator
            let temp_output_dir = tempdir()
                .context("Failed to create temporary output directory")?;
            let output_path = temp_output_dir.path().to_path_buf();

            // Construct the command to execute hf-validator
            let hf_validator_executable = self.hf_validator_path.clone().unwrap_or_else(|| {
                // Fallback to default if not provided in config.toml, assuming release build
                self.args.path.join("target/release/hf-validator")
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

            // Define the permanent output directory
            let permanent_output_dir = PathBuf::from(format!("generated/hf_dataset_output/{}", short_id));
            tokio::fs::create_dir_all(&permanent_output_dir).await?;

            // --- Start: Mapping file logic ---
            use serde::{Deserialize, Serialize};
            use std::collections::HashMap;
            use tokio::fs::File;
            use tokio::io::{AsyncReadExt, AsyncWriteExt};

            #[derive(Debug, Default, Deserialize, Serialize)]
            struct Mapping {
                #[serde(flatten)]
                files: HashMap<String, String>,
            }

            let mapping_file_path = PathBuf::from("generated/hf_dataset_output/mapping.toml");
            let mut mapping = if mapping_file_path.exists() {
                let mut file = File::open(&mapping_file_path).await
                    .context("Failed to open mapping.toml")?;
                let mut contents = String::new();
                file.read_to_string(&mut contents).await
                    .context("Failed to read mapping.toml")?;
                toml::from_str(&contents).context("Failed to parse mapping.toml")?
            } else {
                Mapping::default()
            };

            mapping.files.insert(original_file_path.to_string_lossy().to_string(), short_id.clone());

            let toml_string = toml::to_string_pretty(&mapping).context("Failed to serialize mapping to TOML")?;
            let mut file = File::create(&mapping_file_path).await
                .context("Failed to create mapping.toml")?;
            file.write_all(toml_string.as_bytes()).await
                .context("Failed to write mapping.toml")?;
            // --- End: Mapping file logic ---
            // Copy contents from temporary output directory to permanent directory
            // This requires iterating through the temporary directory and copying each item.
            let mut entries = tokio::fs::read_dir(&output_path).await?;
            while let Some(entry) = entries.next_entry().await? {
                let entry_path = entry.path();
                let destination_path = permanent_output_dir.join(entry_path.file_name().unwrap());
                if entry_path.is_dir() {
                    // Recursively copy directories
                    copy_dir_all(&entry_path, &destination_path).await?;
                } else {
                    tokio::fs::copy(&entry_path, &destination_path).await?;
                }
            }

            // The temporary output directory will be automatically deleted when temp_output_dir goes out of scope
            // We no longer need to forget it as we are copying its contents.

            let __result = Ok(ValidatedFile(source_code, permanent_output_dir));
            measurement::record_function_exit("HuggingFaceValidatorFunctor::map");
            __result
        })
    }
}