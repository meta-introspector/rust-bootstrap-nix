use super::test_split::*;
use crate::prelude::*;
// Build-and-run steps for `./x.py test` test fixtures
//
// `./x.py test` (aka [`Kind::Test`]) is currently allowed to reach build steps in other modules.
// However, this contains ~all test parts we expect people to be able to build and run locally.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::{env, fs, iter};

use clap_complete::shells;

use crate::core::build_steps::doc::DocumentationFormat;
use crate::core::build_steps::synthetic_targets::MirOptPanicAbortSyntheticTarget;
use crate::core::build_steps::tool::{self, SourceType, Tool};
use crate::core::build_steps::toolstate::ToolState;
use crate::core::build_steps::{compile, dist, llvm};
use crate::core::builder::{
    self, Alias, Builder, Compiler, Kind, RunConfig, ShouldRun, Step, crate_description,
};
use crate::core::config::TargetSelection;
use crate::core::config::flags::get_completion;
use crate::Subcommand;
use crate::utils::exec::{BootstrapCommand, command};
use crate::utils::helpers::{
    self, LldThreads, add_link_lib_path, add_rustdoc_cargo_linker_args, dylib_path, dylib_path_var,
    linker_args, linker_flags, t, target_supports_cranelift_backend, up_to_date,
};
use crate::utils::render_tests::{add_flags_and_try_run_tests, try_run_tests};
use crate::{CLang, DocTests, GitRepo, Mode, envify};
use test_definitions_macro::test_definitions;
#[macro_use]
mod test_macros;















mod test_books;
default_test!(Ui { path: "tests/ui", mode: "ui", suite: "ui" });

default_test!(Crashes { path: "tests/crashes", mode: "crashes", suite: "crashes" });

default_test!(Codegen { path: "tests/codegen", mode: "codegen", suite: "codegen" });

default_test!(CodegenUnits {
    path: "tests/codegen-units",
    mode: "codegen-units",
    suite: "codegen-units"
});

default_test!(Incremental { path: "tests/incremental", mode: "incremental", suite: "incremental" });

default_test_with_compare_mode!(Debuginfo {
    path: "tests/debuginfo",
    mode: "debuginfo",
    suite: "debuginfo",
    compare_mode: "split-dwarf"
});

host_test!(UiFullDeps { path: "tests/ui-fulldeps", mode: "ui", suite: "ui-fulldeps" });

host_test!(Rustdoc { path: "tests/rustdoc", mode: "rustdoc", suite: "rustdoc" });
host_test!(RustdocUi { path: "tests/rustdoc-ui", mode: "ui", suite: "rustdoc-ui" });

host_test!(RustdocJson { path: "tests/rustdoc-json", mode: "rustdoc-json", suite: "rustdoc-json" });

host_test!(Pretty { path: "tests/pretty", mode: "pretty", suite: "pretty" });

/// Special-handling is needed for `run-make`, so don't use `default_test` for defining `RunMake`
/// tests.
default_test!(Assembly { path: "tests/assembly", mode: "assembly", suite: "assembly" });

/// Coverage tests are a bit more complicated than other test suites, because
/// we want to run the same set of test files in multiple different modes,
/// in a way that's convenient and flexible when invoked manually.
///
/// This combined step runs the specified tests (or all of `tests/coverage`)
/// in both "coverage-map" and "coverage-run" modes.
///
/// Used by:
/// - `x test coverage`
/// - `x test tests/coverage`
/// - `x test tests/coverage/trivial.rs` (etc)
///
/// (Each individual mode also has its own step that will run the tests in
/// just that mode.)
// Runs `tests/coverage` in "coverage-map" mode only.
// Used by `x test` and `x test coverage-map`.
coverage_test_alias!(CoverageMap {
    alias_and_mode: "coverage-map",
    default: true,
    only_hosts: false,
});
// Runs `tests/coverage` in "coverage-run" mode only.
// Used by `x test` and `x test coverage-run`.
coverage_test_alias!(CoverageRun {
    alias_and_mode: "coverage-run",
    default: true,
    // Compiletest knows how to automatically skip these tests when cross-compiling,
    // but skipping the whole step here makes it clearer that they haven't run at all.
    only_hosts: true,
});

host_test!(CoverageRunRustdoc {
    path: "tests/coverage-run-rustdoc",
    mode: "coverage-run",
    suite: "coverage-run-rustdoc"
});
