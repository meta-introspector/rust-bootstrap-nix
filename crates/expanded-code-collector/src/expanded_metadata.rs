use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpandedMetadata {
    pub package_name: String,
    pub target_type: String,
    pub target_name: String,
    pub cargo_expand_command: String,
    pub timestamp: u64,
    pub flake_lock_details: serde_json::Value,
}
