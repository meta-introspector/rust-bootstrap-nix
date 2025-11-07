use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;
use super::decl_parser::{DeclarationType, TypeUsage};
use anyhow::{Context, Result};
use std::path::Path;
use tokio;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExpandedManifest {
    pub entries: Vec<ExpandedFileEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpandedFileEntry {
    pub package_name: String,
    pub target_type: String,
    pub target_name: String,
    pub expanded_rs_path: PathBuf,
    pub original_path: PathBuf,
    pub rustc_version: String,
    pub rustc_host: String,
    pub cargo_expand_command: String,
    pub timestamp: u64,
    pub flake_lock_details: serde_json::Value,
    pub layer: u32,
    pub file_size: u64,
    pub declaration_counts: HashMap<DeclarationType, usize>,
    pub type_usages: HashMap<String, TypeUsage>,
}

impl ExpandedManifest {
    pub fn add_entry(&mut self, entry: ExpandedFileEntry) {
        self.entries.push(entry);
    }

    pub async fn write_to_file(&self, path: &Path) -> Result<()> {
        let json_content = serde_json::to_string_pretty(&self.entries)
            .context("Failed to serialize expanded manifest to JSON")?;
        tokio::fs::write(path, json_content).await
            .context(format!("Failed to write expanded manifest to file: {}", path.display()))?;
        Ok(())
    }
}
