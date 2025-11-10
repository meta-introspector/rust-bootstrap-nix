// rust-system-composer/src/config_lock.rs

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigLock {
    pub config_toml_hash: String,
    pub stages: HashMap<String, StageLock>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StageLock {
    pub status: StageStatus,
    pub input_hashes: HashMap<String, String>, // e.g., file_path -> hash, data_structure_name -> hash
    pub output_hashes: HashMap<String, String>,
    pub parameters: HashMap<String, String>, // CLI args or config values relevant to this stage
    pub dependencies: Vec<String>, // Names of stages this stage depends on
    pub timestamp: DateTime<Utc>,
    pub log_path: Option<PathBuf>,
    pub report_path: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StageStatus {
    Skipped,
    Cached,
    Executed,
    Failed,
    Pending,
}
