use std::collections::HashMap;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use tokio::fs;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StageStatus {
    Pending,
    Executed,
    Skipped,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageLock {
    pub name: String,
    pub status: StageStatus,
    pub input_hashes: HashMap<String, String>,
    pub output_hashes: HashMap<String, String>,
    pub parameters: HashMap<String, String>,
    pub dependencies: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub log_path: Option<PathBuf>,
    pub report_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigLock {
    pub generated_at: DateTime<Utc>,
    pub config_toml_hash: String,
    pub builder_binary_hash: String, // New field for the builder binary's hash
    pub stages: HashMap<String, StageLock>,
}

impl ConfigLock {
    pub fn new() -> Self {
        ConfigLock {
            generated_at: Utc::now(),
            config_toml_hash: String::new(),
            builder_binary_hash: String::new(), // Initialize new field
            stages: HashMap::new(),
        }
    }

    pub async fn save(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path.parent().unwrap()).await?;
        let serialized = serde_json::to_string_pretty(self)?;
        fs::write(path, serialized).await?;
        Ok(())
    }

    pub async fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        let config_lock: ConfigLock = serde_json::from_str(&content)?;
        Ok(config_lock)
    }

    pub fn get_or_create_stage_lock(&mut self, stage_name: &str) -> StageLock {
        self.stages.entry(stage_name.to_string()).or_insert_with(|| {
            StageLock {
                name: stage_name.to_string(),
                status: StageStatus::Pending,
                input_hashes: HashMap::new(),
                output_hashes: HashMap::new(),
                parameters: HashMap::new(),
                dependencies: Vec::new(),
                timestamp: Utc::now(),
                log_path: None,
                report_path: None,
            }
        }).clone()
    }

    pub fn update_stage_lock(&mut self, stage_lock: StageLock) {
        self.stages.insert(stage_lock.name.clone(), stage_lock);
    }
}

impl Default for ConfigLock {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn calculate_file_hash(file_path: &Path) -> anyhow::Result<String> {
    use tokio::io::AsyncReadExt;

    let mut file = tokio::fs::File::open(file_path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    hasher.update(&buffer);
    Ok(format!("{:x}", hasher.finalize()))
}
    