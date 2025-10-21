use bootstrap::Config;
use bootstrap::TomlConfig;
use bootstrap::dry_run::BUILDER_CONFIG_FILENAME;
use std::path::PathBuf;

pub fn get_builder_toml(config: &Config, build_name: &str) -> Result<TomlConfig, toml::de::Error> {
    if config.dry_run {
        return Ok(TomlConfig::default());
    }

    let builder_config_path =
        config.out.join(config.build.triple).join(build_name).join(BUILDER_CONFIG_FILENAME);
    // Assuming get_toml will also be moved and called as a standalone function
    // For now, I'll keep it as Config::get_toml and fix it later when get_toml is moved.
    Config::get_toml(&builder_config_path)
}