
use crate::parsed_config::ParsedConfig;
use crate::local_toml_config::LocalTomlConfig;
use crate::config_applicator::ConfigApplicator;

pub struct LlvmAssertionsConfigApplicator;

impl ConfigApplicator for LlvmAssertionsConfigApplicator {
    fn apply_to_config(&self, config: &mut ParsedConfig, toml: &LocalTomlConfig) {
        config.llvm_assertions = Some(toml.llvm.as_ref().and_then(|llvm| llvm.assertions).unwrap_or(false));
    }
}
