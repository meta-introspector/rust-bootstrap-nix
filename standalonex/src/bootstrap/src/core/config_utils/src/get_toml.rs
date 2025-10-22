use std::path::Path;
use std::fs;
use serde::Deserialize;

use crate::local_toml_config::LocalTomlConfig;

#[cfg(test)]
pub(crate) fn get_toml(_: &Path) -> Result<LocalTomlConfig, toml::de::Error> {
    Ok(LocalTomlConfig::default())
}

#[cfg(not(test))]
pub(crate) fn get_toml(file: &Path) -> Result<LocalTomlConfig, toml::de::Error> {
    let contents =
        fs::read_to_string(file).expect(&format!("config file {} not found", file.display()));
    // Deserialize to Value and then TomlConfig to prevent the Deserialize impl of
    // TomlConfig and sub types to be monomorphized 5x by toml.
    toml::from_str(&contents)
        .and_then(|table: toml::Value| LocalTomlConfig::deserialize(table))
}
