// This will be the lib.rs for the new bootstrap-config-utils crate
use std::path::{PathBuf, Path};
use std::collections::HashMap;

use serde_derive::Deserialize;
pub mod default_opts;
pub mod get_builder_toml;
pub mod get_toml;
pub mod parse;
pub mod parse_inner_flags;
pub mod parse_inner_src;
pub mod parse_inner_out;
pub mod parse_inner_stage0;
pub mod parse_inner_toml;
pub mod parse_inner_build;
pub mod dry_run;
pub mod try_run;
pub mod ci_config;
pub mod build_config;
pub mod install_config;
pub mod config_applicator;
pub mod llvm_assertions_config;
pub mod rust_channel_git_hash_config;
pub mod local_build;
pub mod local_ci_config;
pub mod local_dist;
pub mod local_flags;
pub mod local_llvm;
pub mod local_rust;
pub mod local_target_config;
pub mod local_toml_config;
pub mod parsed_config;
pub mod target_selection;

