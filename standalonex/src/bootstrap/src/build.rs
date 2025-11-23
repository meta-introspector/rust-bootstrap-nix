use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;
use std::{env, fs, io, str};

use build_helper::ci::gha;
use build_helper::exit;
use build_helper::util::parse_gitmodules;
use cc::Tool;

use crate::core::builder;
use crate::core::builder::{Builder, Kind};
//use crate::core::config::Config;
//use crate::core::config::flags::Flags;
//use crate::core::config::target_selection::TargetSelection;
use crate::core::metadata;
use crate::core::sanity;

use crate::utils::exec::{command, BehaviorOnFailure, BootstrapCommand, CommandOutput, OutputMode};
use crate::utils::helpers::{dir_is_empty, mtime, output, symlink_dir};
use crate::utils::job;
//use crate::{Crate, DocTests, GitInfo, Subcommand};
use termcolor::{ColorChoice, StandardStream, WriteColor};

///
/// This structure transitively contains all configuration for the build system.

pub struct BuilderConfig {
    #[cfg(feature = "build-metrics")]
    metrics: crate::utils::metrics::BuildMetrics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DependencyType {
    /// Libraries originating from proc-macros.
    Host,
    /// Typical Rust libraries.
    Target,
    /// Non Rust libraries and objects shipped to ease usage of certain targets.
    TargetSelfContained,
}

/// The various "modes" of invoking Cargo.
///
/// These entries currently correspond to the various output directories of the
/// build system, with each mod generating output in a different directory.
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    /// Build the standard library, placing output in the "stageN-std" directory.
    Std,

    /// Build librustc, and compiler libraries, placing output in the "stageN-rustc" directory.
    Rustc,

    /// Build a codegen backend for rustc, placing the output in the "stageN-codegen" directory.
    Codegen,

    /// Build a tool, placing output in the "stage0-bootstrap-tools"
    /// directory. This is for miscellaneous sets of tools that are built
    /// using the bootstrap stage0 compiler in its entirety (target libraries
    /// and all). Typically these tools compile with stable Rust.
    ///
    /// Only works for stage 0.
    ToolBootstrap,
}

forward! {
    verbose(f: impl Fn()),
    is_verbose() -> bool,
    create(path: &Path, s: &str),
    remove(f: &Path),
    tempdir() -> PathBuf,
    llvm_link_shared() -> bool,
    download_rustc() -> bool,
    initial_rustfmt() -> Option<PathBuf>,
    last_modified_commit(modified_paths: &[&str], option_name: &str, if_unchanged: bool) -> Option<String>,
    needs_sanitizer_runtime_built(target: TargetSelection) -> bool,
    llvm_libunwind(target: TargetSelection) -> LlvmLibunwind,
    ci_llvm_root() -> PathBuf,
    profiler_path(target: TargetSelection) -> Option<&str>,
    profiler_enabled(target: TargetSelection) -> bool,
}
