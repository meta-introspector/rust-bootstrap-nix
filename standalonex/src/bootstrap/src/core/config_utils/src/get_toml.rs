use bootstrap::TomlConfig;
use std::path::Path;
use toml;
use std::fs;
use build_helper::exit;
use bootstrap::ChangeIdWrapper;
use bootstrap::t;

#[cfg(test)]
pub(crate) fn get_toml(_: &Path) -> Result<TomlConfig, toml::de::Error> {
    Ok(TomlConfig::default())
}

#[cfg(not(test))]
pub(crate) fn get_toml(file: &Path) -> Result<TomlConfig, toml::de::Error> {
    let contents =
        t!(fs::read_to_string(file), format!("config file {} not found", file.display()));
    // Deserialize to Value and then TomlConfig to prevent the Deserialize impl of
    // TomlConfig and sub types to be monomorphized 5x by toml.
    toml::from_str(&contents)
        .and_then(|table: toml::Value| TomlConfig::deserialize(table))
        .inspect_err(|_| {
            if let Ok(Some(changes)) = toml::from_str(&contents)
                .and_then(|table: toml::Value| ChangeIdWrapper::deserialize(table))
                .map(|change_id| change_id.inner.map(bootstrap::find_recent_config_change_ids))
            {
                if !changes.is_empty() {
                    println!(
                        "WARNING: There have been changes to x.py since you last updated:\n{}",
                        bootstrap::human_readable_changes(&changes)
                    );
                }
            }
        })
}