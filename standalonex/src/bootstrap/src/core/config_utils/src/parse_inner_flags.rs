use crate::parsed_config::ParsedConfig;
use crate::local_flags::LocalFlags;

pub fn parse_inner_flags(_config: &mut ParsedConfig, _flags: &mut LocalFlags) {
    // These fields are no longer part of LocalFlags and are handled elsewhere.
}