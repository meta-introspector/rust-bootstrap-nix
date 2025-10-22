use std::path::PathBuf;
use crate::parsed_config::ParsedConfig;
use crate::local_toml_config::LocalTomlConfig;
use crate::config_applicator::ConfigApplicator;

pub struct CiConfigApplicator;

impl ConfigApplicator for CiConfigApplicator {
    fn apply_to_config(&self, config: &mut ParsedConfig, toml: &LocalTomlConfig) {
        let ci_config = toml.ci.clone().unwrap_or_default();
        config.channel_file = ci_config.channel_file;
        config.version_file = ci_config.version_file;
        config.tools_dir = ci_config.tools_dir;
        config.llvm_project_dir = ci_config.llvm_project_dir;
        config.gcc_dir = ci_config.gcc_dir;

//        config.change_id = toml.change_id.inner;
    }
}