use crate::prelude::*



use crate::parsed_config::ParsedConfig;
use crate::local_toml_config::LocalTomlConfig;
use crate::target_selection::TargetSelection;
use crate::get_toml;
use crate::dry_run::DryRun;

const BUILDER_CONFIG_FILENAME: &str = "config.toml";

pub fn get_builder_toml(config: &ParsedConfig, build_name: &str) -> Result<LocalTomlConfig, toml::de::Error> {
    if config.dry_run != DryRun::Disabled {
        return Ok(LocalTomlConfig::default());
    }

    let TargetSelection(ref build_triple) = config.build;
    let builder_config_path =
        config.out.join(build_triple).join(build_name).join(BUILDER_CONFIG_FILENAME);
    // Assuming get_toml will also be moved and called as a standalone function
    // For now, I'll keep it as Config::get_toml and fix it later when get_toml is moved.
    get_toml::get_toml(&builder_config_path)
}
