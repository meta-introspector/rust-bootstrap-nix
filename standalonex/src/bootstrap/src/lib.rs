mod paths;
pub use paths::*;

/// Main entry point for the Rust bootstrap build system.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Display;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::time::SystemTime;
use std::{env, io, str};

use build_helper::ci::gha;
use build_helper::exit;
use sha2::digest::Digest;
use termcolor::{ColorChoice, StandardStream, WriteColor};
use utils::channel::GitInfo;
use utils::helpers::hex_encode;


//pub use crate::core::builder::*;
//use crate::core::builder::{Builder, Kind};

pub use bootstrap_config_types::*; // Added this line

pub use crate::utils::exec::{BehaviorOnFailure, BootstrapCommand, CommandOutput, OutputMode, command};
pub use crate::utils::helpers::{
    self, dir_is_empty, exe, libdir, mtime, output, set_file_times, symlink_dir,
};

mod core;
pub mod utils;
pub use utils::*;
pub mod prelude;
pub mod krate;
pub use self::krate::Crate;
pub mod mode;
pub use self::mode::Mode;
pub mod build;
// pub use crate::core::config::BuildConfig; // Removed this line
pub mod compiler;
pub use self::compiler::Compiler;
pub mod dependency_type;
pub use self::dependency_type::DependencyType;
#[macro_use]
mod macros;
mod constants;
pub use self::constants::*;
// pub use crate::core::config::DocTests; // Removed this line
// pub use crate::core::config::DocTests; // Removed this line (duplicate)
pub use crate::utils::cc_detect::Language;

pub use crate::utils::envify;
pub use crate::utils::generate_smart_stamp_hash;
pub use crate::utils::prepare_behaviour_dump_dir;
//pub use crate::Subcommand;

pub use utils::change_tracker::{CONFIG_CHANGE_HISTORY, find_recent_config_change_ids, human_readable_changes,};
