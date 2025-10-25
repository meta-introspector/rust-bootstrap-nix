use crate::prelude::*


use std::path::Path;

use build_helper::util;
use build_helper::exit;
use crate::Build;
use crate::core::config::Config;
use crate::utils::channel::GitInfo;
use crate::utils::helpers::{self, dir_is_empty};

impl Build {
    /// Updates a submodule, and exits with a failure if submodule management
    /// is disabled and the submodule does not exist.
    ///
    /// The given submodule name should be its path relative to the root of
    /// the main repository.
    ///
    /// The given `err_hint` will be shown to the user if the submodule is not
    /// checked out and submodule management is disabled.
    pub fn require_submodule(&self, submodule: &str, err_hint: Option<&str>) {
        // When testing bootstrap itself, it is much faster to ignore
        // submodules. Almost all Steps work fine without their submodules.
        if cfg!(test) && !self.config.submodules() {
            return;
        }
        self.config.update_submodule(submodule);
        let absolute_path = self.config.src.join(submodule);
        if dir_is_empty(&absolute_path) {
            let maybe_enable = if !self.config.submodules()
                && self.config.rust_info.is_managed_git_subrepository()
            {
                "\nConsider setting `build.submodules = true` or manually initializing the submodules."
            } else {
                ""
            };
            let err_hint = err_hint.map_or_else(String::new, |e| format!("\n{e}"));
            eprintln!(
                "submodule {submodule} does not appear to be checked out, "
                 "but it is required for this step{maybe_enable}{err_hint}"
            );
            exit!(1);
        }
    }

    /// Updates all submodules, and exits with an error if submodule
    /// management is disabled and the submodule does not exist.
    pub fn require_and_update_all_submodules(&self) {
        for submodule in util::parse_gitmodules(&self.src) {
            self.require_submodule(submodule, None);
        }
    }

    /// If any submodule has been initialized already, sync it unconditionally.
    /// This avoids contributors checking in a submodule change by accident.
    pub fn update_existing_submodules(&self) {
        // Avoid running git when there isn't a git checkout, or the user has
        // explicitly disabled submodules in `config.toml`.
        if !self.config.submodules() {
            return;
        }
        let output = helpers::git(Some(&self.src))
            .args(["config", "--file"])
            .arg(".gitmodules")
            .args(["--get-regexp", "path"])
            .run_capture(self)
            .stdout();
        std::thread::scope(|s| {
            // Look for `submodule.$name.path = $path`
            // Sample output: `submodule.src/rust-installer.path src/tools/rust-installer`
            for line in output.lines() {
                let submodule = line.split_once(' ').unwrap().1;
                let config = self.config.clone();
                s.spawn(move || {
                    Self::update_existing_submodule(&config, submodule);
                });
            }
        });
    }

    /// Updates the given submodule only if it's initialized already; nothing happens otherwise.
    pub fn update_existing_submodule(config: &Config, submodule: &str) {
        // Avoid running git when there isn't a git checkout.
        if !config.submodules() {
            return;
        }

        if GitInfo::new(false, Path::new(submodule)).is_managed_git_subrepository() {
            config.update_submodule(submodule);
        }
    }
}
