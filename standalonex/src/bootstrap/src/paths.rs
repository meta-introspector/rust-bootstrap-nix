use std::path::{Path, PathBuf};
use std::env;
use std::process::Command;
use std::sync::OnceLock;
use crate::Build;
use crate::Compiler;
use crate::Mode;
use crate::TargetSelection;
use crate::core::config::Target;
use crate::utils::helpers::{exe, t, output, libdir};
use crate::builder::BootstrapCommand;

    fn local_path(&self, build: &Build) -> PathBuf {
        self.path.strip_prefix(&build.config.src).unwrap().into()
    }
    fn cargo_dir(&self) -> &'static str {
        if self.config.rust_optimize.is_release() { "release" } else { "debug" }
    }

    fn tools_dir(&self, compiler: Compiler) -> PathBuf {
        let out = self.out.join(compiler.host).join(format!("stage{}-tools-bin", compiler.stage));
        t!(fs::create_dir_all(&out));
        out
    }

    /// Returns the root directory for all output generated in a particular
    /// stage when running with a particular host compiler.
    ///
    /// The mode indicates what the root directory is for.
    fn stage_out(&self, compiler: Compiler, mode: Mode) -> PathBuf {
        let suffix = match mode {
            Mode::Std => "-std",
            Mode::Rustc => "-rustc",
            Mode::Codegen => "-codegen",
            Mode::ToolBootstrap => "-bootstrap-tools",
            Mode::ToolStd | Mode::ToolRustc => "-tools",
        };
        self.out.join(compiler.host).join(format!("stage{}{}", compiler.stage, suffix))
    }

    /// Returns the root output directory for all Cargo output in a given stage,
    /// running a particular compiler, whether or not we're building the
    /// standard library, and targeting the specified architecture.
    fn cargo_out(&self, compiler: Compiler, mode: Mode, target: TargetSelection) -> PathBuf {
        self.stage_out(compiler, mode).join(target).join(self.cargo_dir())
    }

    /// Root output directory of LLVM for `target`
    ///
    /// Note that if LLVM is configured externally then the directory returned
    /// will likely be empty.
    fn llvm_out(&self, target: TargetSelection) -> PathBuf {
        if self.config.llvm_from_ci && self.config.build == target {
            self.config.ci_llvm_root()
        } else {
            self.out.join(target).join("llvm")
        }
    }

    fn enzyme_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(&*target.triple).join("enzyme")
    }

    fn gcc_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(&*target.triple).join("gcc")
    }

    fn lld_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(target).join("lld")
    }

    /// Output directory for all documentation for a target
    fn doc_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(target).join("doc")
    }

    /// Output directory for all JSON-formatted documentation for a target
    fn json_doc_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(target).join("json-doc")
    }

    fn test_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(target).join("test")
    }

    /// Output directory for all documentation for a target
    fn compiler_doc_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(target).join("compiler-doc")
    }

    /// Output directory for some generated md crate documentation for a target (temporary)
    fn md_doc_out(&self, target: TargetSelection) -> PathBuf {
        self.out.join(target).join("md-doc")
    }

    /// Returns `true` if this is an external version of LLVM not managed by bootstrap.
    /// In particular, we expect llvm sources to be available when this is false.
    ///
    /// NOTE: this is not the same as `!is_rust_llvm` when `llvm_has_patches` is set.
    fn is_system_llvm(&self, target: TargetSelection) -> bool {
        match self.config.target_config.get(&target) {
            Some(Target { llvm_config: Some(_), .. }) => {
                let ci_llvm = self.config.llvm_from_ci && target == self.config.build;
                !ci_llvm
            }
            // We're building from the in-tree src/llvm-project sources.
            Some(Target { llvm_config: None, .. }) => false,
            None => false,
        }
    }

    /// Returns `true` if this is our custom, patched, version of LLVM.
    ///
    /// This does not necessarily imply that we're managing the `llvm-project` submodule.
    fn is_rust_llvm(&self, target: TargetSelection) -> bool {
        match self.config.target_config.get(&target) {
            // We're using a user-controlled version of LLVM. The user has explicitly told us whether the version has our patches.
            // (They might be wrong, but that's not a supported use-case.)
            // In particular, this tries to support `submodules = false` and `patches = false`, for using a newer version of LLVM that's not through `rust-lang/llvm-project`.
            Some(Target { llvm_has_rust_patches: Some(patched), .. }) => *patched,
            // The user hasn't promised the patches match.
            // This only has our patches if it's downloaded from CI or built from source.
            _ => !self.is_system_llvm(target),
        }
    }

    /// Returns the path to llvm/bin
    fn llvm_bin(&self, target: TargetSelection) -> PathBuf {
        let target_config = self.config.target_config.get(&target);
        if let Some(s) = target_config.and_then(|c| c.llvm_config.as_ref()) {
            let llvm_bindir = output(Command::new(s).arg("--bindir"));
            PathBuf::from(llvm_bindir.trim())
        } else {
            self.llvm_out(self.config.build).join("bin")
        }
    }

    /// Returns the path to `FileCheck` binary for the specified target
    fn llvm_filecheck(&self, target: TargetSelection) -> PathBuf {
        let target_config = self.config.target_config.get(&target);
        if let Some(s) = target_config.and_then(|c| c.llvm_filecheck.as_ref()) {
            s.to_path_buf()
        } else if let Some(s) = target_config.and_then(|c| c.llvm_config.as_ref()) {
            let llvm_bindir = command(s).arg("--bindir").run_capture_stdout(self).stdout();
            let filecheck = Path::new(llvm_bindir.trim()).join(exe("FileCheck", target));
            if filecheck.exists() {
                filecheck
            } else {
                // On Fedora the system LLVM installs FileCheck in the
                // llvm subdirectory of the libdir.
                let llvm_libdir = command(s).arg("--libdir").run_capture_stdout(self).stdout();
                let lib_filecheck =
                    Path::new(llvm_libdir.trim()).join("llvm").join(exe("FileCheck", target));
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
            base.join("bin").join(exe("FileCheck", target))
        }
    }

    /// Directory for libraries built from C/C++ code and shared between stages.
    fn native_dir(&self, target: TargetSelection) -> PathBuf {
        self.out.join(target).join("native")
    }

    /// Root output directory for rust_test_helpers library compiled for
    /// `target`
    fn test_helpers_out(&self, target: TargetSelection) -> PathBuf {
        self.native_dir(target).join("rust-test-helpers")
    }

    /// Adds the `RUST_TEST_THREADS` env var if necessary
    fn add_rust_test_threads(&self, cmd: &mut BootstrapCommand) {
        if env::var_os("RUST_TEST_THREADS").is_none() {
            cmd.env("RUST_TEST_THREADS", self.jobs().to_string());
        }
    }

    /// Returns the libdir of the snapshot compiler.
    fn rustc_snapshot_libdir(&self) -> PathBuf {
        self.rustc_snapshot_sysroot().join(libdir(self.config.build))
    }

    /// Returns the sysroot of the snapshot compiler.
    fn rustc_snapshot_sysroot(&self) -> &Path {
        static SYSROOT_CACHE: OnceLock<PathBuf> = OnceLock::new();
