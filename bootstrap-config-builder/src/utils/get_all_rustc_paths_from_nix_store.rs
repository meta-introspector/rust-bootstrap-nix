use crate::prelude::*


use anyhow::{Context, Result};
use std::process::Command;
use log::{info, debug};
use crate::utils::extract_rustc_version_from_path::extract_rustc_version_from_path;
//use crate::utils::nix_eval_utils; // Import the new module
//use bootstrap_config_builder::utils::nix_eval_utils::run_nix_eval;

pub fn get_all_rustc_paths_from_nix_store() -> Result<Vec<(String, String)>> {
    info!("Executing: ls /nix/store/*/bin/rustc");
    let mut command = Command::new("sh");
    command.args(&["-c", "ls /nix/store/*/bin/rustc"]);

    let output = command.output()
        .with_context(|| "Failed to execute ls /nix/store/*/bin/rustc")?;

    if !output.status.success() {
        debug!(
            "ls /nix/store/*/bin/rustc failed:\n{}\nStderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout.lines()
        .map(|s| {
            let path = s.trim().to_string();
            let version = extract_rustc_version_from_path(&path);
            (path, version)
        })
        .collect())
}

