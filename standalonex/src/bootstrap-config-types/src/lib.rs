//use build_helper::util::{output as build_helper_output, exe as build_helper_exe};
use std::process::Command;
use crate::target_selection::TargetSelection;

fn output(cmd: &mut Command) -> String {
    let stdout = build_helper_output(cmd);
    String::from_utf8(stdout).expect("command output was not valid utf-8")
}

fn exe(name: &str, target: crate::target_selection::TargetSelection) -> String {
    build_helper_exe(name, target)
}

fn is_download_ci_available(_triple: &TargetSelection, _llvm_assertions: bool) -> bool {
    false
}

//use crate::prelude::*;
pub mod prelude;
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DocTests {
    Yes,
    No,
    Only,
}
