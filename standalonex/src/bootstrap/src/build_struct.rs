use crate::prelude::*


use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::path::{PathBuf};

use build_helper::ci::gha;
use crate::core::config::Config;
use crate::core::config::target_selection::TargetSelection;
use crate::enums::{DocTests, GitRepo};
use crate::crate_struct::Crate;

/// Global configuration for the build system.
///
/// This structure transitively contains all configuration for the build system.
/// All filesystem-encoded configuration is in `config`, all flags are in
/// `flags`, and then parsed or probed information is listed in the keys below.
///
/// This structure is a parameter of almost all methods in the build system,
/// although most functions are implemented as free functions rather than
/// methods specifically on this structure itself (to make it easier to
/// organize).
#[derive(Clone)]
pub struct Build {
    /// User-specified configuration from `config.toml`.
    pub config: Config,

    // Version information
    pub version: String,

    // Properties derived from the above configuration
    pub src: PathBuf,
    pub out: PathBuf,
    pub bootstrap_out: PathBuf,
    pub cargo_info: GitInfo,
    pub rust_analyzer_info: GitInfo,
    pub clippy_info: GitInfo,
    pub miri_info: GitInfo,
    pub rustfmt_info: GitInfo,
    pub enzyme_info: GitInfo,
    pub in_tree_llvm_info: GitInfo,
    pub in_tree_gcc_info: GitInfo,
    pub local_rebuild: bool,
    pub fail_fast: bool,
    pub doc_tests: DocTests,
    pub verbosity: usize,

    /// Build triple for the pre-compiled snapshot compiler.
    pub build: TargetSelection,
    /// Which triples to produce a compiler toolchain for.
    pub hosts: Vec<TargetSelection>,
    /// Which triples to build libraries (core/alloc/std/test/proc_macro) for.
    pub targets: Vec<TargetSelection>,

    pub initial_rustc: PathBuf,
    pub initial_cargo: PathBuf,
    pub initial_lld: PathBuf,
    pub initial_libdir: PathBuf,
    pub initial_sysroot: PathBuf,

    // Runtime state filled in later on
    // C/C++ compilers and archiver for all targets
    pub cc: RefCell<HashMap<TargetSelection, cc::Tool>>,
    pub cxx: RefCell<HashMap<TargetSelection, cc::Tool>>,
    pub ar: RefCell<HashMap<TargetSelection, PathBuf>>,
    pub ranlib: RefCell<HashMap<TargetSelection, PathBuf>>,
    // Miscellaneous
    // allow bidirectional lookups: both name -> path and path -> name
    pub crates: HashMap<String, Crate>,
    pub crate_paths: HashMap<PathBuf, String>,
    pub is_sudo: bool,
    pub delayed_failures: RefCell<Vec<String>>,
    pub prerelease_version: Cell<Option<u32>>,

    #[cfg(feature = "build-metrics")]
    pub metrics: crate::utils::metrics::BuildMetrics,
}
