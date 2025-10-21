//! Serialized configuration of a build.
//!
//! This module implements parsing `config.toml` configuration files to tweak
//! how the build runs.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::{self, Display};
use std::io::IsTerminal;
use std::path::{Path, PathBuf, absolute};
use std::process::Command;
use std::str::FromStr;
use std::sync::OnceLock;
use std::{cmp, env, fs};

use build_helper::ci::CiEnv;
use build_helper::exit;
use build_helper::git::{GitConfig, get_closest_merge_commit, output_result};
use serde::{Deserialize, Deserializer};
use serde_derive::Deserialize;

use crate::core::build_steps::compile::CODEGEN_BACKEND_PREFIX;
use crate::core::build_steps::llvm;
pub use crate::core::config::flags::Subcommand;
use crate::core::config::flags::{Color, Flags, Warnings};
use crate::core::download::is_download_ci_available;
use crate::utils::cache::{INTERNER, Interned};
use crate::utils::channel::{self, GitInfo};
use crate::utils::helpers::{self, exe, output, t};

/// Each path in this list is considered "allowed" in the `download-rustc="if-unchanged"` logic.
/// This means they can be modified and changes to these paths should never trigger a compiler build
/// when "if-unchanged" is set.
///
/// NOTE: Paths must have the ":!" prefix to tell git to ignore changes in those paths during
/// the diff check.
///
/// WARNING: Be cautious when adding paths to this list. If a path that influences the compiler build
/// is added here, it will cause bootstrap to skip necessary rebuilds, which may lead to risky results.
/// For example, "src/bootstrap" should never be included in this list as it plays a crucial role in the
/// final output/compiler, which can be significantly affected by changes made to the bootstrap sources.
#[rustfmt::skip] // We don't want rustfmt to oneline this list
pub(crate) const RUSTC_IF_UNCHANGED_ALLOWED_PATHS: &[&str] = &[
    ":!src/tools",
    ":!tests",
    ":!triagebot.toml",
];

macro_rules! check_ci_llvm {
    ($name:expr) => {
        assert!(
            $name.is_none(),
            "setting {} is incompatible with download-ci-llvm.",
            stringify!($name).replace("_", "-")
        );
    };
}


pub use target_selection;


 // use build_helper::ci::CiEnv;
 // use build_helper::exit;
 // use build_helper::git::get_closest_merge_commit;
 // use build_helper::git::GitConfig;
 // use build_helper::git::output_result;
 // use cc::Build;
 // use clap::builder::styling::Color;
 // use clap::Command;
 // use clap::Subcommand;
 // use clap::ValueEnum;
 // use cmake::Config;
 // use crate::BTreeSet;
 // use crate::Build;
 // use crate::Cell;
 // use crate::Command;
 // use crate::core::build_steps::compile::CODEGEN_BACKEND_PREFIX;
 // use crate::core::build_steps::llvm;
 // use crate::core::build_steps::llvm::Llvm;
 // use crate::core::build_steps::setup::Profile;
 // use crate::core::build_steps::setup::Profile::Dist;
 // use crate::core::build_steps::tool::LibcxxVersion::Llvm;

use changeid::ChangeIdWrapper;
use ciconfig::CiConfig;
use color::Color;
use config_base::Config;
use debug_info_level::DebuginfoLevel;
use dry_run::BUILDER_CONFIG_FILENAME;
use rustclto::RustcLto;
use rustfmt::RustfmtState;
use rust_optimize::RustOptimize;
use splitdebuginfo::SplitDebuginfo;
use stringorbool::StringOrBool;
use subcommand::get_completion;
use subcommand::Subcommand;
use subcommand::Subcommand::Build;
use subcommand::Subcommand::Dist;
use subcommand::Subcommand::Install;
use tomlconfig::TomlConfig;
use warnings::Warnings;

// use crate::core::download::is_download_ci_available;
 // use crate::define_config;
 // use crate::Display;
 // use crate::DocTests;
pub use dry_run::*;
 // use crate::env;
 // use crate::exe;
 // use crate::exit;
 // use crate::Flags;
 // use crate::fs;
 // use crate::GitInfo;
 // use crate::GitRepo::Llvm;
 // use crate::HashMap;
 // use crate::HashSet;
 // use crate::helpers;
 // use crate::Kind;
 // use crate::Kind::Build;
 // use crate::Kind::Dist;
 // use crate::Kind::Install;
pub use lld_mode::*;
 // use crate::LlvmLibunwind;
 // use crate::OnceLock;
 // use crate::output;
 // use crate::Path;
 // use crate::PathBuf;
 // use crate::RefCell;
 // use crate::str::FromStr;
 // use crate::t;
 // use crate::Target;
pub use target_selection::TargetSelection;
 // use crate::utils::cache::Interned;
 // use crate::utils::cache::INTERNER;
 // use crate::utils::channel;
 // use crate::utils::shared_helpers::exe;
 // use crate::utils::tarball::OverlayKind::Llvm;
 // use crate::utils::tarball::OverlayKind::Rust;
 // use serde_derive::Deserialize;
 // use serde::Deserialize;
 // use serde::Deserializer;
 // use std::cell::Cell;
 // use std::cell::RefCell;
 // use std::cmp;
 // use std::collections::BTreeSet;
 // use std::collections::HashMap;
 // use std::collections::HashSet;
 // use std::env;
 // use std::fmt;
 // use std::fmt::Display;
 // use std::fs;
 // use std::path::absolute;
 // use std::path::Path;
 // use std::path::PathBuf;
 // use std::process::Command;
 // use std::str::FromStr;
 // use std::sync::OnceLock;
 // use termcolor::Color;
