use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Default)]
pub struct NixConfig {
    #[serde(default)]
    pub nixpkgs_path: String,
    // Add other nix-related fields as needed
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub nix: NixConfig,
    // Add other top-level sections as needed
}

pub fn parse_config(config_path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}
