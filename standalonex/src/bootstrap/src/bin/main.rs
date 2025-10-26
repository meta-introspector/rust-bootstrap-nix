use crate::prelude::*;


//! bootstrap, the Rust build system
//!
//! This is the entry point for the build system used to compile the `rustc`
//! compiler. Lots of documentation can be found in the `README.md` file in the
//! parent directory, and otherwise documentation can be found throughout the `build`
//! directory in each respective module.

use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, IsTerminal, Write};
use std::str::FromStr;
use std::{env, process};

use bootstrap::{Build, CONFIG_CHANGE_HISTORY, Config, Flags, Subcommand, find_recent_config_change_ids, human_readable_changes, t, prelude::*};
use bootstrap_config_utils::parse;
use bootstrap_config_utils::dry_run;
use build_helper::ci::CiEnv;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if Flags::try_parse_verbose_help(&args) {
        return;
    }

use crate::prelude::*;


//! bootstrap, the Rust build system
//!
//! This is the entry point for the build system used to compile the `rustc`
//! compiler. Lots of documentation can be found in the `README.md` file in the
//! parent directory, and otherwise documentation can be found throughout the `build`
//! directory in each respective module.

use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, IsTerminal, Write};
use std::str::FromStr;
use std::{env, process};

use bootstrap::{Build, CONFIG_CHANGE_HISTORY, Config, Flags, Subcommand, find_recent_config_change_ids, human_readable_changes, t, prelude::*};
use bootstrap_config_utils::parse;
use bootstrap_config_utils::dry_run;
use build_helper::ci::CiEnv;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if Flags::try_parse_verbose_help(&args) {
        return;
    }

    let flags = Flags::parse(&args);
    let mut config = parse::parse(flags);

    // Resolve Nix paths dynamically if not already set
    config.resolve_nix_paths().expect("Failed to resolve Nix paths");

    let mut build_results = Vec::new();

    for rustc_version in &config.rustc_versions {
        for cargo_version in &config.cargo_versions {
            let mut new_config = config.clone();
            new_config.initial_rustc = PathBuf::from(rustc_version);
            new_config.initial_cargo = PathBuf::from(cargo_version);

            println!("Building with rustc: {} and cargo: {}", rustc_version, cargo_version);

            let mut build_lock;
            let _build_lock_guard;

            if !new_config.bypass_bootstrap_lock {
                // Display PID of process holding the lock
                // PID will be stored in a lock file
                let lock_path = new_config.out.join("lock");
                let pid = fs::read_to_string(&lock_path);

                build_lock = fd_lock::RwLock::new(t!(fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(&lock_path)));
                _build_lock_guard = match build_lock.try_write() {
                    Ok(mut lock) => {
                        t!(lock.write(process::id().to_string().as_ref()));
                        lock
                    }
                    err => {
                        drop(err);
                        if let Ok(pid) = pid {
                            println!("WARNING: build directory locked by process {pid}, waiting for lock");
                        } else {
                            println!("WARNING: build directory locked, waiting for lock");
                        }
                        let mut lock = t!(build_lock.write());
                        t!(lock.write(process::id().to_string().as_ref()));
                        lock
                    }
                };
            }

            let build_result = std::panic::catch_unwind(|| {
                Build::new(new_config).build();
            });

            build_results.push((rustc_version.clone(), cargo_version.clone(), build_result.is_ok()));
        }
    }

    println!("Build results:");
    for (rustc_version, cargo_version, success) in &build_results {
        println!("  rustc: {}, cargo: {}, success: {}", rustc_version, cargo_version, success);
    }
}

fn check_version(config: &Config) -> Option<String> {
    let mut msg = String::new();

    let latest_change_id = CONFIG_CHANGE_HISTORY.last().unwrap().change_id;
    let warned_id_path = config.out.join("bootstrap").join(".last-warned-change-id");

    if let Some(mut id) = config.change_id {
        if id == latest_change_id {
            return None;
        }

        // Always try to use `change-id` from .last-warned-change-id first. If it doesn't exist,
        // then use the one from the config.toml. This way we never show the same warnings
        // more than once.
        if let Ok(t) = fs::read_to_string(&warned_id_path) {
            let last_warned_id = usize::from_str(&t)
                .unwrap_or_else(|_| panic!("{} is corrupted.", warned_id_path.display()));

            // We only use the last_warned_id if it exists in `CONFIG_CHANGE_HISTORY`.
            // Otherwise, we may retrieve all the changes if it's not the highest value.
            // For better understanding, refer to `change_tracker::find_recent_config_change_ids`.
            if CONFIG_CHANGE_HISTORY.iter().any(|config| config.change_id == last_warned_id) {
                id = last_warned_id;
            }
        };

        let changes = find_recent_config_change_ids(id);

        if changes.is_empty() {
            return None;
        }

        msg.push_str("There have been changes to x.py since you last updated:\n");
        msg.push_str(&human_readable_changes(&changes));

        msg.push_str("NOTE: to silence this warning, ");
        msg.push_str(&format!(
            "update `config.toml` to use `change-id = {latest_change_id}` instead"
        ));

        if io::stdout().is_terminal() && !dry_run::dry_run(&config) {
            t!(fs::write(warned_id_path, latest_change_id.to_string()));
        }
    } else {
        msg.push_str("WARNING: The `change-id` is missing in the `config.toml`. This means that you will not be able to track the major changes made to the bootstrap configurations.\n");
        msg.push_str("NOTE: to silence this warning, ");
        msg.push_str(&format!("add `change-id = {latest_change_id}` at the top of `config.toml`"));
    };

    Some(msg)
}


fn check_version(config: &Config) -> Option<String> {
    let mut msg = String::new();

    let latest_change_id = CONFIG_CHANGE_HISTORY.last().unwrap().change_id;
    let warned_id_path = config.out.join("bootstrap").join(".last-warned-change-id");

    if let Some(mut id) = config.change_id {
        if id == latest_change_id {
            return None;
        }

        // Always try to use `change-id` from .last-warned-change-id first. If it doesn't exist,
        // then use the one from the config.toml. This way we never show the same warnings
        // more than once.
        if let Ok(t) = fs::read_to_string(&warned_id_path) {
            let last_warned_id = usize::from_str(&t)
                .unwrap_or_else(|_| panic!("{} is corrupted.", warned_id_path.display()));

            // We only use the last_warned_id if it exists in `CONFIG_CHANGE_HISTORY`.
            // Otherwise, we may retrieve all the changes if it's not the highest value.
            // For better understanding, refer to `change_tracker::find_recent_config_change_ids`.
            if CONFIG_CHANGE_HISTORY.iter().any(|config| config.change_id == last_warned_id) {
                id = last_warned_id;
            }
        };

        let changes = find_recent_config_change_ids(id);

        if changes.is_empty() {
            return None;
        }

        msg.push_str("There have been changes to x.py since you last updated:\n");
        msg.push_str(&human_readable_changes(&changes));

        msg.push_str("NOTE: to silence this warning, ");
        msg.push_str(&format!(
            "update `config.toml` to use `change-id = {latest_change_id}` instead"
        ));

        if io::stdout().is_terminal() && !dry_run::dry_run(&config) {
            t!(fs::write(warned_id_path, latest_change_id.to_string()));
        }
    } else {
        msg.push_str("WARNING: The `change-id` is missing in the `config.toml`. This means that you will not be able to track the major changes made to the bootstrap configurations.\n");
        msg.push_str("NOTE: to silence this warning, ");
        msg.push_str(&format!("add `change-id = {latest_change_id}` at the top of `config.toml`"));
    };

    Some(msg)
}
