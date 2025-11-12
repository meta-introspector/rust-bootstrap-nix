use crate::prelude::*;
use serde_derive::Deserialize;
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
    pub nix: Option<LocalNixConfig>,
}
