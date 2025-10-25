use crate::prelude::*


use crate::parsed_config::ParsedConfig;
use std::path::{Path, PathBuf};
use std::env;

pub fn parse_inner_out(config: &mut ParsedConfig) {
    if cfg!(test) {
        // Use the build directory of the original x.py invocation, so that we can set `initial_rustc` properly.
        config.out = env::var_os("CARGO_TARGET_DIR")
            .map(|s| Path::new(&s).parent().unwrap().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("target/test_output")); // Provide a default for tests
    }
}
