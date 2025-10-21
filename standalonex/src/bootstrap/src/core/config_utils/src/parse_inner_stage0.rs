use bootstrap::Config;
use bootstrap::TomlConfig;
use build_helper;

pub fn parse_inner_stage0(config: &mut Config, toml: &TomlConfig) {
    config.stage0_metadata = build_helper::stage0_parser::parse_stage0_file(
        &toml.stage0_path.as_ref().expect("stage0_path must be set"),
    );
}