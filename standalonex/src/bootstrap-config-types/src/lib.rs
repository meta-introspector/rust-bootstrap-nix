pub enum Kind {
    Bench,
    Check,
    Clippy,
    Fix,
    Format,
    Test,
    Miri,
    Suggest,
    Perf,
    Build,
    Doc,
    Dist,
    Install,
    Clean,
    Run,
    Setup,
    Vendor,
}

pub enum DocTests {
    Only,
    No,
    Yes,
}

use bootstrap_macros::t;

use std::process::Command;
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cell::{Cell, RefCell};
use std::cmp;
use std::env;
use std::fs;
use std::fmt::{self, Display};
use std::str::FromStr;
use std::path::{Path, PathBuf};

pub use clap::{Parser, ValueEnum, Args, Subcommand};

pub use build_helper::git::{GitConfig, output_result as git_output_result, get_closest_merge_commit };
//GitInfo, output

pub use build_helper::ci::CiEnv;
pub use build_helper::exit;
pub use build_helper::channel;
//pub use build_helper::get_toml;
pub use build_helper::get_builder_toml;
pub use build_helper::RUSTC_IF_UNCHANGED_ALLOWED_PATHS;
pub use build_helper::helpers;

pub use crate::target_selection::{TargetSelection, TargetSelectionList, Target};
pub use crate::tomlconfig::{TomlConfig};
pub use crate::flags::Flags;
pub use crate::rust_optimize::RustOptimize;
pub use crate::debug_info_level::DebuginfoLevel;
pub use crate::lld_mode::LldMode;
pub use crate::rustclto::RustcLto;
pub use crate::llvm_lib_unwind::LlvmLibunwind;
pub use crate::splitdebuginfo::SplitDebuginfo;
pub use crate::stringorbool::StringOrBool;
pub use crate::string_or_int::StringOrInt;
pub use crate::rustfmt::RustfmtState;
pub use crate::replaceop::ReplaceOpt;
pub use crate::changeid::ChangeIdWrapper;
//pub use crate::ci::Ci;
pub use crate::subcommand::DistTool::Dist;
pub use crate::subcommand::DistTool::Install;
//pub use crate::llvm::Llvm;
//pub use crate::rust::Rust;
//pub use crate::build::Build;
pub use crate::Kind::Build;
//pub use crate::subcommand::BuildTool::Build;
//pub use crate::subcommand::BuildTool::Build;
pub use crate::warnings::Warnings;
pub use crate::color::Color;
pub use crate::dry_run::DryRun;
pub use crate::config_base::Config;
pub use crate::config_part2::check_incompatible_options_for_ci_rustc;
pub use crate::config_part6::OptimizeVisitor;
pub use config_macros::define_config;

fn output(cmd: &mut Command) -> Vec<u8> {
    cmd.output().expect("command failed to run").stdout
}

fn exe(name: &str, _target: crate::target_selection::TargetSelection) -> String {
    if cfg!(windows) {
        format!("{}.exe", name)
    } else {
        name.to_string()
    }
}

fn is_download_ci_available(_triple: &TargetSelection, _llvm_assertions: bool) -> bool {
    false
}

const CODEGEN_BACKEND_PREFIX: &str = "codegen-backend-";

pub mod build;
pub mod changeid;
pub mod ci;
pub mod ciconfig;
pub mod color;
pub mod config;
pub mod config_base;
pub mod config_ci;
pub mod config_part2;
pub mod config_part3;
pub mod config_part4;
pub mod config_part6;
pub mod config_part7;
pub mod config_toml;
pub mod config_types;
pub mod config_utils;
pub mod debug_info_level;
pub mod dist;
pub mod dry_run;
pub mod flags;
pub mod install;
pub mod lld_mode;
pub mod llvm;
pub mod llvm_lib_unwind;
#[macro_use]
pub mod macro_rules;
pub mod merge;
pub mod replaceop;
pub mod rust;
pub mod rust_optimize;
pub mod rustclto;
pub mod rustfmt;
pub mod splitdebuginfo;
pub mod string_or_int;
pub mod stringorbool;
pub mod subcommand;
pub mod subcommand_groups;
pub mod target;
pub mod target_selection;
pub mod tests;
pub mod tomlconfig;
pub mod tomltarget;
pub mod warnings;