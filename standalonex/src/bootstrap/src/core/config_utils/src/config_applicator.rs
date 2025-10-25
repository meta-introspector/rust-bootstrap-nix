use crate::prelude::*


use crate::parsed_config::ParsedConfig;
use crate::local_toml_config::LocalTomlConfig;
pub trait ConfigApplicator {
    fn apply_to_config(&self, config: &mut ParsedConfig, toml: &LocalTomlConfig);
}
