use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub bins: Option<HashMap<String, String>>,
    #[serde(rename = "generatedOutputDir")]
    pub generated_output_dir: Option<PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            bins: None,
            generated_output_dir: None,
        }
    }
}

pub fn read_config(config_path: &Path, project_root: &Path) -> Result<Config> {
    let full_config_path = project_root.join(config_path);
    let config_content = std::fs::read_to_string(&full_config_path)
        .with_context(|| format!("Failed to read config file from {}", full_config_path.display()))?;

    let config: Config = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse config file from {}", full_config_path.display()))?;

    Ok(config)
}
