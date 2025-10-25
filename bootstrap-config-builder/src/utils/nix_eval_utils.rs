use crate::prelude::*


use anyhow::{Context, Result};
use std::process::Command;
use log::debug;

pub fn run_nix_eval(expr: &str) -> Result<String> {
    let mut command = Command::new("nix");
    command.args(&["eval", "--raw", "--extra-experimental-features", "nix-command flakes", "--expr", expr]);

    debug!("Executing Nix eval command: {:?}", command);

    let output = command.output()
        .with_context(|| format!("Failed to execute nix eval for expression: '{}'", expr))?;

    if !output.status.success() {
        anyhow::bail!(
            "Nix eval command failed for expression '{}':\n{}
Stderr: {}",
            expr,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8(output.stdout)?.trim().to_string();
    debug!("Nix eval stdout: {}", stdout);
    Ok(stdout)
}
