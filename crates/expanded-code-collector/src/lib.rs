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



// The expand_macros function has been moved to expander.rs

pub async fn collect_expanded_code(
    metadata_path: &Path,
    output_dir: &Path,
    flake_lock_json: &serde_json::Value,
    layer: Option<u32>,
    package_filter: Option<String>,
    dry_run: bool,
    force: bool,
    rustc_version: String,
    rustc_host: String,
) -> Result<()> {
    println!("Collecting expanded code...");

    fs::create_dir_all(output_dir).await
        .context(format!("Failed to create output directory: {}", output_dir.display()))?;

    let mut error_collection = ErrorCollection::default();

    let expanded_files_entries = expander::expand_all_packages(
        metadata_path,
        output_dir,
        &flake_lock_json,
        layer,
        package_filter,
        dry_run,
        force,
        &mut error_collection,
        rustc_version.clone(),
        rustc_host.clone(),
    ).await?;

    let mut expanded_manifest = manifest::ExpandedManifest::default();

    for (entry, _content) in expanded_files_entries {
        expanded_manifest.add_entry(manifest::ExpandedFileEntry {
            package_name: entry.package_name,
            target_type: entry.target_type,
            target_name: entry.target_name,
            expanded_rs_path: entry.expanded_rs_path,
            cargo_expand_command: entry.cargo_expand_command,
            timestamp: entry.timestamp,
            flake_lock_details: entry.flake_lock_details,
            layer: entry.layer,
            file_size: entry.file_size,
            declaration_counts: entry.declaration_counts,
            type_usages: entry.type_usages,
            original_path: entry.original_path, // Use the actual original_path
            rustc_version: entry.rustc_version, // Use the actual rustc_version
            rustc_host: entry.rustc_host,       // Use the actual rustc_host
        });
    }

    if !dry_run {
        let errors_path = output_dir.join("collected_errors.json");
        error_collection.write_to_file(&errors_path).await?;
        let manifest_path = output_dir.join("expanded_manifest.json");
        expanded_manifest.write_to_file(&manifest_path).await?;
    }

    Ok(())
}
