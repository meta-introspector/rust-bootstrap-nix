use build_helper::prelude::*;
use build_helper::prelude::*;
use build_helper::exit;
use bootstrap_macros::t;
use std::cell::*;
use std::collections::*;
use build_helper::prelude::*;
use crate::TargetSelection;
use crate::Target;
use crate::flags::Color;
use crate::subcommand::Subcommand;
use crate::DryRun;
use crate::LldMode;
use crate::RustOptimize;
use crate::DebuginfoLevel;
use build_helper::channel;
use crate::RustfmtState;
use crate::RustcLto;
use crate::LlvmLibunwind;

use crate::ciconfig::CiConfig;
use crate::TomlConfig;
use crate::rust::Rust;
use crate::llvm::Llvm;
use crate::Config;
use crate::Flags;

use crate::env;
use crate::fs;
use std::collections::HashMap;
use config_core::ReplaceOpt;

/// Compares the current `Llvm` options against those in the CI LLVM builder and detects any incompatible options.
/// It does this by destructuring the `Llvm` instance to make sure every `Llvm` field is covered and not missing.
#[cfg(not(feature = "bootstrap-self-test"))]
pub(crate) fn check_incompatible_options_for_ci_llvm(
    current_config_toml: TomlConfig,
    ci_config_toml: TomlConfig,
) -> Result<(), String> {
    macro_rules! err {
        ($current:expr, $expected:expr) => {
            if let Some(current) = &$current { if Some(current) != $expected .as_ref() {
            return
            Err(format!("ERROR: Setting `llvm.{}` is incompatible with `llvm.download-ci-llvm`. \
                        Current value: {:?}, Expected value(s): {}{:?}",
            stringify!($expected) .replace("_", "-"), $current, if $expected .is_some() {
            "None/" } else { "" }, $expected,)); }; };
        };
    }
    macro_rules! warn {
        ($current:expr, $expected:expr) => {
            if let Some(current) = &$current { if Some(current) != $expected .as_ref() {
            println!("WARNING: `llvm.{}` has no effect with `llvm.download-ci-llvm`. \
                        Current value: {:?}, Expected value(s): {}{:?}",
            stringify!($expected) .replace("_", "-"), $current, if $expected .is_some() {
            "None/" } else { "" }, $expected,); }; };
        };
    }
    let (Some(current_llvm_config), Some(ci_llvm_config)) = (
        current_config_toml.llvm,
        ci_config_toml.llvm,
    ) else {
        return Ok(());
    };
    let Llvm {
        optimize,
        thin_lto,
        release_debuginfo,
        assertions: _,
        tests: _,
        plugins,
        ccache: _,
        static_libstdcpp: _,
        libzstd,
        ninja: _,
        targets,
        experimental_targets,
        link_jobs: _,
        link_shared: _,
        version_suffix,
        clang_cl,
        cflags,
        cxxflags,
        ldflags,
        use_libcxx,
        use_linker,
        allow_old_toolchain,
        offload,
        polly,
        clang,
        enable_warnings,
        download_ci_llvm: _,
        build_config,
        enzyme,
        enable_projects: _,
    } = ci_llvm_config;
    err!(current_llvm_config.optimize, optimize);
    err!(current_llvm_config.thin_lto, thin_lto);
    err!(current_llvm_config.release_debuginfo, release_debuginfo);
    err!(current_llvm_config.libzstd, libzstd);
    err!(current_llvm_config.targets, targets);
    err!(current_llvm_config.experimental_targets, experimental_targets);
    err!(current_llvm_config.clang_cl, clang_cl);
    err!(current_llvm_config.version_suffix, version_suffix);
    err!(current_llvm_config.cflags, cflags);
    err!(current_llvm_config.cxxflags, cxxflags);
    err!(current_llvm_config.ldflags, ldflags);
    err!(current_llvm_config.use_libcxx, use_libcxx);
    err!(current_llvm_config.use_linker, use_linker);
    err!(current_llvm_config.allow_old_toolchain, allow_old_toolchain);
    err!(current_llvm_config.offload, offload);
    err!(current_llvm_config.polly, polly);
    err!(current_llvm_config.clang, clang);
    err!(current_llvm_config.build_config, build_config);
    err!(current_llvm_config.plugins, plugins);
    err!(current_llvm_config.enzyme, enzyme);
    warn!(current_llvm_config.enable_warnings, enable_warnings);
    Ok(())
}
