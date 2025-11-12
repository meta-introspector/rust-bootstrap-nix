use std::path::PathBuf;

use crate::Build;
use crate::Compiler;
use crate::core::config::target_selection::TargetSelection;

impl Build {
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
