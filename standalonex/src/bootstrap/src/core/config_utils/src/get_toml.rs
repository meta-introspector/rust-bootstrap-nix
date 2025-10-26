use crate::prelude::*;
#[cfg(test)]
pub(crate) fn get_toml(_: &Path) -> Result<LocalTomlConfig, toml::de::Error> {
    Ok(LocalTomlConfig::default())
}
#[cfg(not(test))]
pub(crate) fn get_toml(file: &Path) -> Result<LocalTomlConfig, toml::de::Error> {
    let contents = fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("config file {} not found", file.display()));
    toml::from_str(&contents)
        .and_then(|table: toml::Value| LocalTomlConfig::deserialize(table))
}
