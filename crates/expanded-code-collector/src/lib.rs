use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use tokio::fs;
use serde::{Serialize, Deserialize};
use chrono::Utc;

mod expander;
mod metadata;
mod manifest;
mod decl_parser;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RustcInfo {
    pub version: String,
    pub host: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorSample {
    pub file_path: PathBuf,
    pub rustc_version: String,
    pub rustc_host: String,
    pub error_type: String,
    pub error_message: String,
    pub code_snippet: Option<String>,
    pub timestamp: chrono::DateTime<Utc>,
    pub context: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ErrorCollection {
    pub errors: Vec<ErrorSample>,
}

impl ErrorCollection {
    pub fn add_error(&mut self, error: ErrorSample) {
        self.errors.push(error);
    }

    pub async fn write_to_file(&self, path: &Path) -> Result<()> {
        let json_content = serde_json::to_string_pretty(&self.errors)
            .context("Failed to serialize error collection to JSON")?;
        fs::write(path, json_content).await
            .context(format!("Failed to write error collection to file: {}", path.display()))?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpandedManifestEntry {
    pub package_name: String,
    pub file_path: PathBuf,
    pub expanded_path: PathBuf,
    pub original_path: PathBuf,
    pub rustc_version: String,
    pub rustc_host: String,
    pub layer: u32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExpandedManifest {
    pub entries: Vec<ExpandedManifestEntry>,
}

impl ExpandedManifest {
    pub fn add_entry(&mut self, entry: ExpandedManifestEntry) {
        self.entries.push(entry);
    }

    pub async fn write_to_file(&self, path: &Path) -> Result<()> {
        let json_content = serde_json::to_string_pretty(&self.entries)
            .context("Failed to serialize expanded manifest to JSON")?;
        fs::write(path, json_content).await
            .context(format!("Failed to write expanded manifest to file: {}", path.display()))?;
        Ok(())
    }
}

// The expand_macros function has been moved to expander.rs

pub async fn collect_expanded_code(
    metadata_path: &Path,
    output_dir: &Path,
    layer: Option<u32>,
    package_filter: Option<String>,
    dry_run: bool,
) -> Result<()> {
    println!("Collecting expanded code...");

    fs::create_dir_all(output_dir).await
        .context(format!("Failed to create output directory: {}", output_dir.display()))?;

    // Dummy flake_lock_json for now, as collect_expanded_code doesn't have access to it.
    let flake_lock_json = serde_json::json!({});

    let expanded_files_entries = expander::expand_all_packages(
        metadata_path,
        output_dir,
        &flake_lock_json,
        layer,
        package_filter,
        dry_run,
    ).await?;

    let mut expanded_manifest = ExpandedManifest::default();

    for (entry, _content) in expanded_files_entries {
        expanded_manifest.add_entry(ExpandedManifestEntry {
            package_name: entry.package_name,
            file_path: entry.expanded_rs_path.clone(), // Using expanded_rs_path as file_path for now
            expanded_path: entry.expanded_rs_path,
            original_path: PathBuf::from("unknown"), // Original path is not directly available here
            rustc_version: "unknown".to_string(), // Not directly available from ExpandedFileEntry
            rustc_host: "unknown".to_string(), // Not directly available from ExpandedFileEntry
            layer: entry.layer,
        });
    }

    if !dry_run {
        // ErrorCollection is not directly populated by expander::expand_all_packages
        // For now, we will not write collected_errors.json
        // let errors_path = output_dir.join("collected_errors.json");
        // error_collection.write_to_file(&errors_path).await?;
        let manifest_path = output_dir.join("expanded_manifest.json");
        expanded_manifest.write_to_file(&manifest_path).await?;
    }

    Ok(())
}
