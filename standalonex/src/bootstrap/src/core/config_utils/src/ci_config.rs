use std::path::PathBuf;
use crate::ParsedConfig;
use crate::LocalTomlConfig;
use crate::ConfigApplicator;

pub struct CiConfigApplicator;

impl ConfigApplicator for CiConfigApplicator {
    fn apply_to_config(&self, config: &mut ParsedConfig, toml: &LocalTomlConfig) {
        let ci_config = toml.ci.clone().unwrap_or_default();
        config.channel_file = ci_config.channel_file.map(PathBuf::from);
        config.version_file = ci_config.version_file.map(PathBuf::from);
        config.tools_dir = ci_config.tools_dir.map(PathBuf::from);
        config.llvm_project_dir = ci_config.llvm_project_dir.map(PathBuf::from);
        config.gcc_dir = ci_config.gcc_dir.map(PathBuf::from);

//        config.change_id = toml.change_id.inner;
    }
}