use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use super::utils::copy_dir_all;
use pipeline_traits::ValidatedFile; // Assuming ValidatedFile is part of pipeline_traits

#[derive(Debug, Default, Deserialize, Serialize)]
struct Mapping {
    #[serde(flatten)]
    files: HashMap<String, String>,
}

pub struct ValidationResultManager;

impl ValidationResultManager {
    pub async fn manage_results(
        source_code: String, // Added source_code here
        output_path: &Path, // Temporary output path from hf-validator
        original_file_path: &Path,
        short_id: &str,
        generated_output_dir: &Path,
    ) -> Result<ValidatedFile> {
        // Define the permanent output directory
        let permanent_output_dir = generated_output_dir.join(format!("hf_dataset_output/{}", short_id));
        tokio::fs::create_dir_all(&permanent_output_dir).await?;

        // --- Start: Mapping file logic ---
        let mapping_file_path = generated_output_dir.join("hf_dataset_output/mapping.toml");
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

        mapping.files.insert(original_file_path.to_string_lossy().to_string(), short_id.to_string());

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

        Ok(ValidatedFile(source_code, permanent_output_dir))
    }
}
