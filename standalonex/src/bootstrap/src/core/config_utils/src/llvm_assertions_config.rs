use crate::ParsedConfig;
use crate::LocalTomlConfig;
use crate::ConfigApplicator;

pub struct LlvmAssertionsConfigApplicator;

impl ConfigApplicator for LlvmAssertionsConfigApplicator {
    fn apply_to_config(&self, config: &mut ParsedConfig, toml: &LocalTomlConfig) {
        config.llvm_assertions = toml.llvm.as_ref().and_then(|llvm| llvm.assertions).unwrap_or(false);
    }
}
