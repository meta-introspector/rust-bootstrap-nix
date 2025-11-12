use crate::prelude::*;
const BUILDER_CONFIG_FILENAME: &str = "config.toml";
pub fn get_builder_toml(
    config: &ParsedConfig,
    build_name: &str,
) -> Result<LocalTomlConfig, toml::de::Error> {
    if config.dry_run != DryRun::Disabled {
        return Ok(LocalTomlConfig::default());
    }
    let TargetSelection(ref build_triple) = config.build;
    let builder_config_path = config
        .out
        .join(build_triple)
        .join(build_name)
        .join(BUILDER_CONFIG_FILENAME);
    get_toml::get_toml(&builder_config_path)
}
