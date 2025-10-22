use serde_derive::Deserialize;
use crate::local_ci_config::LocalCiConfig;
use crate::local_build::LocalBuild;
use crate::local_llvm::LocalLlvm;
use crate::local_rust::LocalRust;
use crate::local_target_config::LocalTargetConfig;
use crate::local_dist::LocalDist;
use crate::install_config;

#[derive(Debug, Default, Deserialize)]
#[derive(Clone)]
pub struct LocalTomlConfig {
    pub ci: Option<LocalCiConfig>,
    pub build: Option<LocalBuild>,
    pub llvm: Option<LocalLlvm>,
    pub rust: Option<LocalRust>,
    pub target: Option<std::collections::HashMap<String, LocalTargetConfig>>,
    pub install: Option<install_config::Install>,
    pub dist: Option<LocalDist>,
    // ... other fields will go here
}
