use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;
use crate::decl_parser::DeclarationType;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpandedManifest {
    pub rustc_version: String,
    pub rustc_host: String,
    pub project_root: PathBuf,
    pub expanded_files: Vec<ExpandedFileEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpandedFileEntry {
    pub package_name: String,
    pub target_type: String,
    pub target_name: String,
    pub expanded_rs_path: PathBuf,
    pub cargo_expand_command: String,
    pub timestamp: u64,
    pub flake_lock_details: serde_json::Value,
    pub layer: u32,
    pub file_size: u64,
    pub declaration_counts: HashMap<DeclarationType, usize>,
}
