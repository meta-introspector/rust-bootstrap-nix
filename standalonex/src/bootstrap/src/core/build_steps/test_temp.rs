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

mod common_test_fields;

const ADB_TEST_DIR: &str = "/data/local/tmp/work";

mod test_steps;

fn check_if_tidy_is_installed(builder: &Builder<'_>) -> bool {
    command("tidy").allow_failure().arg("--version").run_capture_stdout(builder).is_success()
}

fn path_for_cargo(builder: &Builder<'_>, compiler: Compiler) -> OsString {
    // Configure PATH to find the right rustc. NB. we have to use PATH
    // and not RUSTC because the Cargo test suite has tests that will
    // fail if rustc is not spelled `rustc`.
    let path = builder.sysroot(compiler).join("bin");
    let old_path = env::var_os("PATH").unwrap_or_default();
    env::join_paths(iter::once(path).chain(env::split_paths(&old_path))).expect("")
}

fn get_browser_ui_test_version_inner(
    builder: &Builder<'_>,
    npm: &Path,
    global: bool,
) -> Option<String> {
    let mut command = command(npm);
    command.arg("list").arg("--parseable").arg("--long").arg("--depth=0");
    if global {
        command.arg("--global");
    }
    let lines = command.allow_failure().run_capture(builder).stdout();
    lines
        .lines()
        .find_map(|l| l.split(':').nth(1)?.strip_prefix("browser-ui-test@"))
        .map(|v| v.to_owned())
}

fn get_browser_ui_test_version(builder: &Builder<'_>, npm: &Path) -> Option<String> {
    get_browser_ui_test_version_inner(builder, npm, false)
        .or_else(|| get_browser_ui_test_version_inner(builder, npm, true))
}

fn testdir(builder: &Builder<'_>, host: TargetSelection) -> PathBuf {
    builder.out.join(host).join("test")
}