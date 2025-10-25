use crate::prelude::*


use std::collections::{BTreeSet, HashSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::process::Command;

use crate::Build;
use crate::Compiler;
use crate::GitRepo;
use crate::core::config::lld_mode::LldMode;
use crate::core::config::llvm_lib_unwind::LlvmLibunwind;
use crate::core::config::target_selection::TargetSelection;
use crate::utils::channel::GitInfo;
use crate::utils::helpers::{output, libdir};

impl Build {
    /// Gets the space-separated set of activated features for the standard library.
    /// This can be configured with the `std-features` key in config.toml.
    pub fn std_features(&self, target: TargetSelection) -> String {
        let mut features: BTreeSet<&str> =
            self.config.rust_std_features.iter().map(|s| s.as_str()).collect();

        match self.config.llvm_libunwind(target) {
            LlvmLibunwind::InTree => features.insert("llvm-libunwind"),
            LlvmLibunwind::System => features.insert("system-llvm-libunwind"),
            LlvmLibunwind::No => false,
        };

        if self.config.backtrace {
            features.insert("backtrace");
        }
        if self.config.profiler_enabled(target) {
            features.insert("profiler");
        }
        // Generate memcpy, etc.  FIXME: Remove this once compiler-builtins
        // automatically detects this target.
        if target.contains("zkvm") {
            features.insert("compiler-builtins-mem");
        }

        features.into_iter().collect::<Vec<_>>().join(" ")
    }

    /// Gets the space-separated set of activated features for the compiler.
    pub fn rustc_features(&self, kind: crate::core::builder::Kind, target: TargetSelection, crates: &[String]) -> String {
        let possible_features_by_crates: HashSet<_> = crates
            .iter()
            .flat_map(|krate| &self.crates[krate].features)
            .map(std::ops::Deref::deref)
            .collect();
        let check = |feature: &str| -> bool {
            crates.is_empty() || possible_features_by_crates.contains(feature)
        };
        let mut features = vec![];
        if self.config.jemalloc && check("jemalloc") {
            features.push("jemalloc");
        }
        if (self.config.llvm_enabled(target) || kind == crate::core::builder::Kind::Check) && check("llvm") {
            features.push("llvm");
        }
        // keep in sync with `bootstrap/compile.rs:rustc_cargo_env`
        if self.config.rust_randomize_layout {
            features.push("rustc_randomized_layouts");
        }

        // If debug logging is on, then we want the default for tracing:
        // https://github.com/tokio-rs/tracing/blob/3dd5c03d907afdf2c39444a29931833335171554/tracing/src/level_filters.rs#L26
        // which is everything (including debug/trace/etc.)
        // if its unset, if debug_assertions is on, then debug_logging will also be on
        // as well as tracing *ignoring* this feature when debug_assertions is on
        if !self.config.rust_debug_logging && check("max_level_info") {
            features.push("max_level_info");
        }

        features.join(" ")
    }

    /// Component directory that Cargo will produce output into (e.g.
    /// release/debug)
    pub fn cargo_dir(&self) -> &'static str {
        if self.config.rust_optimize.is_release() { "release" } else { "debug" }
    }

    pub fn tools_dir(&self, compiler: Compiler) -> PathBuf {
        let out = self.out.join(compiler.host).join(format!("stage{}-tools-bin", compiler.stage));
        t!(fs::create_dir_all(&out));
        out
    }

    /// Returns the root directory for all output generated in a particular
    /// stage when running with a particular host compiler.
    ///
    /// The mode indicates what the root directory is for.
    pub fn stage_out(&self, compiler: Compiler, mode: crate::Mode) -> PathBuf {
        let suffix = match mode {
            crate::Mode::Std => "-std",
            crate::Mode::Rustc => "-rustc",
            crate::Mode::Codegen => "-codegen",
            crate::Mode::ToolBootstrap => "-bootstrap-tools",
            crate::Mode::ToolStd | crate::Mode::ToolRustc => "-tools",
        };
        self.out.join(compiler.host).join(format!("stage{}{}", compiler.stage, suffix))
    }

    /// Returns the root output directory for all Cargo output in a given stage,
    /// running a particular compiler, whether or not we're building the
    /// standard library, and targeting the specified architecture.
    pub fn cargo_out(&self, compiler: Compiler, mode: crate::Mode, target: TargetSelection) -> PathBuf {
        self.stage_out(compiler, mode).join(target).join(self.cargo_dir())
    }

    /// Root output directory of LLVM for `target`
    ///
    /// Note that if LLVM is configured externally then the directory returned
    /// will likely be empty.
    pub fn llvm_out(&self, target: TargetSelection) -> PathBuf {
        if self.config.llvm_from_ci && self.config.build == target {
            self.config.ci_llvm_root()
        } else {
            self.out.join(target).join("llvm")
        }
    }

    pub fn enzyme_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(&*target.triple).join("enzyme")
    }

    pub fn gcc_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(&*target.triple).join("gcc")
    }

    pub fn lld_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(target).join("lld")
    }

    /// Output directory for all documentation for a target
    pub fn doc_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(target).join("doc")
    }

    /// Output directory for all JSON-formatted documentation for a target
    pub fn json_doc_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(target).join("json-doc")
    }

    pub fn test_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(target).join("test")
    }

    /// Output directory for all documentation for a target
    pub fn compiler_doc_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(target).join("compiler-doc")
    }

    /// Output directory for some generated md crate documentation for a target (temporary)
    pub fn md_doc_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(target).join("md-doc")
    }

    /// Returns `true` if this is an external version of LLVM not managed by bootstrap.
    /// In particular, we expect llvm sources to be available when this is false.
    ///
    /// NOTE: this is not the same as `!is_rust_llvm` when `llvm_has_patches` is set.
    pub fn is_system_llvm(&self, target: TargetSelection) -> bool {
        match self.config.target_config.get(&target) {
            Some(crate::core::config::Target { llvm_config: Some(_), .. }) => {
                let ci_llvm = self.config.llvm_from_ci && target == self.config.build;
                !ci_llvm
            }
            // We're building from the in-tree src/llvm-project sources.
            Some(crate::core::config::Target { llvm_config: None, .. }) => false,
            None => false,
        }
    }

    /// Returns `true` if this is our custom, patched, version of LLVM.
    ///
    /// This does not necessarily imply that we're managing the `llvm-project` submodule.
    pub fn is_rust_llvm(&self, target: TargetSelection) -> bool {
        match self.config.target_config.get(&target) {
            // We're using a user-controlled version of LLVM. The user has explicitly told us whether the version has our patches.
            // (They might be wrong, but that's not a supported use-case.)
            // In particular, this tries to support `submodules = false` and `patches = false`, for using a newer version of LLVM that's not through `rust-lang/llvm-project`.
            Some(crate::core::config::Target { llvm_has_rust_patches: Some(patched), .. }) => *patched,
            // The user hasn't promised the patches match.
            // This only has our patches if it's downloaded from CI or built from source.
            _ => !self.is_system_llvm(target),
        }
    }

    /// Returns the path to llvm/bin
    pub fn llvm_bin(&self, target: TargetSelection) -> PathBuf {
        let target_config = self.config.target_config.get(&target);
        if let Some(s) = target_config.and_then(|c| c.llvm_config.as_ref()) {
            let llvm_bindir = output(Command::new(s).arg("--bindir"));
            PathBuf::from(llvm_bindir.trim())
        } else {
            self.llvm_out(self.config.build).join("bin")
        }
    }

    /// Returns the path to `FileCheck` binary for the specified target
    pub fn llvm_filecheck(&self, target: TargetSelection) -> PathBuf {
        let target_config = self.config.target_config.get(&target);
        if let Some(s) = target_config.and_then(|c| c.llvm_filecheck.as_ref()) {
            s.to_path_buf()
        } else if let Some(s) = target_config.and_then(|c| c.llvm_config.as_ref()) {
            let llvm_bindir = crate::utils::exec::command(s).arg("--bindir").run_capture_stdout(self).stdout();
            let filecheck = Path::new(llvm_bindir.trim()).join(crate::utils::helpers::exe("FileCheck", target));
            if filecheck.exists() {
                filecheck
            } else {
                // On Fedora the system LLVM installs FileCheck in the
                // llvm subdirectory of the libdir.
                let llvm_libdir = crate::utils::exec::command(s).arg("--libdir").run_capture_stdout(self).stdout();
                let lib_filecheck =
                    Path::new(llvm_libdir.trim()).join("llvm").join(crate::utils::helpers::exe("FileCheck", target));
                if lib_filecheck.exists() {
                    lib_filecheck
                } else {
                    // Return the most normal file name, even though
                    // it doesn't exist, so that any error message
                    // refers to that.
                    filecheck
                }
            }
        } else {
            let base = self.llvm_out(target).join("build");
            let base = if !self.ninja() && target.is_msvc() {
                if self.config.llvm_optimize {
                    if self.config.llvm_release_debuginfo {
                        base.join("RelWithDebInfo")
                    } else {
                        base.join("Release")
                    }
                } else {
                    base.join("Debug")
                }
            } else {
                base
            };
            base.join("bin").join(crate::utils::helpers::exe("FileCheck", target))
        }
    }

    /// Directory for libraries built from C/C++ code and shared between stages.
    pub fn native_dir(&self, target: TargetSelection) -> PathBuf {
        self.out.join(target).join("native")
    }

    /// Root output directory for rust_test_helpers library compiled for
    /// `target`
    pub fn test_helpers_out(&self, target: TargetSelection) -> PathBuf {
        self.native_dir(target).join("rust-test-helpers")
    }

    /// Returns the libdir of the snapshot compiler.
    pub fn rustc_snapshot_libdir(&self) -> PathBuf {
        self.rustc_snapshot_sysroot().join(libdir(self.config.build))
    }

    /// Returns the sysroot of the snapshot compiler.
    pub fn rustc_snapshot_sysroot(&self) -> &Path {
        static SYSROOT_CACHE: OnceLock<PathBuf> = OnceLock::new();
        SYSROOT_CACHE.get_or_init(|| {
            let mut rustc = Command::new(&self.initial_rustc);
            rustc.args(["--print", "sysroot"]);
            output(&mut rustc).trim().into()
        })
    }
}