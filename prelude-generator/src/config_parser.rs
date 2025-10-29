use serde::Deserialize;
use std::path::PathBuf;
use anyhow::{Result, Context};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct BinsConfig {
    #[serde(flatten)]
    pub paths: HashMap<String, PathBuf>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bins: BinsConfig,
}

pub fn read_config(config_path: &PathBuf) -> Result<Config> {
    let config_content = std::fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
    let config: Config = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;
    Ok(config)
}
