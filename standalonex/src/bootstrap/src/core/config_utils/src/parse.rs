
use crate::parsed_config::ParsedConfig;
use std::path::{Path, PathBuf};
use std::env;
use toml;

use crate::parse_inner_flags;
use crate::parse_inner_stage0;
use crate::parse_inner_toml;
use crate::parse_inner_src;
use crate::parse_inner_out;
use crate::config_applicator::ConfigApplicator;
use crate::ci_config;
use crate::build_config;
use crate::install_config;
use crate::llvm_assertions_config;
use crate::rust_channel_git_hash_config;

use crate::local_flags::LocalFlags;
use crate::local_toml_config::LocalTomlConfig;
use crate::get_toml;

pub fn parse(mut flags: LocalFlags) -> ParsedConfig {
    let mut config = ParsedConfig::default();

    // Set flags.
    parse_inner_flags::parse_inner_flags(&mut config, &mut flags);

    // Infer the rest of the configuration.
    let build_src_from_toml = None; // This needs to be handled differently if it's coming from toml.build.src
    parse_inner_src::parse_inner_src(&mut config, &flags, &build_src_from_toml);

    parse_inner_out::parse_inner_out(&mut config);

    let mut toml = parse_inner_toml::parse_inner_toml(&mut config, &flags, get_toml::get_toml);

    // Apply various configuration applicators
    let mut applicators: Vec<Box<dyn ConfigApplicator>> = Vec::new();
    applicators.push(Box::new(ci_config::CiConfigApplicator));
    applicators.push(Box::new(build_config::BuildConfigApplicator));
    applicators.push(Box::new(install_config::InstallConfigApplicator));
    applicators.push(Box::new(llvm_assertions_config::LlvmAssertionsConfigApplicator));
    applicators.push(Box::new(rust_channel_git_hash_config::RustChannelGitHashConfigApplicator));

    for applicator in applicators.iter() {
        applicator.apply_to_config(&mut config, &toml);
    }

    config
}

fn apply_test_config(config: &mut ParsedConfig, toml: &mut LocalTomlConfig) {
    if cfg!(test) {
        let build = toml.build.get_or_insert_with(Default::default);
        build.rustc = build.rustc.take().or(std::env::var_os("RUSTC").map(|p| p.into()));
        build.cargo = build.cargo.take().or(std::env::var_os("CARGO").map(|p| p.into()));
    }
}