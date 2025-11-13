use crate::prelude::*;
//extern crate serde;
use serde::Deserialize;
use serde::Serialize;
//pub use serde :: { Deserialize , Serialize } ;
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct NixConfig {
    #[serde(default)]
    pub nixpkgs_path: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub nix: NixConfig,
}
pub fn parse_config(
    config_path: &PathBuf,
) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}
