use crate::prelude::*;


use std::path::PathBuf;
use std::process::Command;

use crate::Build;
use crate::CLang;
use crate::GitRepo;
use crate::core::config::lld_mode::LldMode;
use crate::core::config::target_selection::TargetSelection;
use crate::core::sanity;
use crate::utils::exec::{BootstrapCommand, command};
use crate::utils::helpers::{exe, libdir, output};

impl Build {
    /// Returns the path to the C compiler for the target specified.
    pub fn cc(&self, target: TargetSelection) -> PathBuf {
        if self.config.dry_run {
            return PathBuf::new();
        }
        self.cc.borrow()[&target].path().into()
    }

    /// Returns a list of flags to pass to the C compiler for the target
    /// specified.
    pub fn cflags(&self, target: TargetSelection, which: GitRepo, c: CLang) -> Vec<String> {
        if self.config.dry_run {
            return Vec::new();
        }
        let base = match c {
            CLang::C => self.cc.borrow()[&target].clone(),
            CLang::Cxx => self.cxx.borrow()[&target].clone(),
        };

        // Filter out -O and /O (the optimization flags) that we picked up from
        // cc-rs because the build scripts will determine that for themselves.
        let mut base = base
            .args()
            .iter()
            .map(|s| s.to_string_lossy().into_owned())
            .filter(|s| !s.starts_with("-O") && !s.starts_with("/O"))
            .collect::<Vec<String>>();

        // If we're compiling C++ on macOS then we add a flag indicating that
        // we want libc++ (more filled out than libstdc++), ensuring that
        // LLVM/etc are all properly compiled.
        if matches!(c, CLang::Cxx) && target.contains("apple-darwin") {
            base.push("-stdlib=libc++".into());
        }

        // Work around an apparently bad MinGW / GCC optimization,
        // See: https://lists.llvm.org/pipermail/cfe-dev/2016-December/051980.html
        // See: https://gcc.gnu.org/bugzilla/show_bug.cgi?id=78936
        if &*target.triple == "i686-pc-windows-gnu" {
            base.push("-fno-omit-frame-pointer".into());
        }

        if let Some(map_to) = self.debuginfo_map_to(which) {
            let map = format!("{}=", self.src.display(), map_to);
            let cc = self.cc(target);
            if cc.ends_with("clang") || cc.ends_with("gcc") {
                base.push(format!(