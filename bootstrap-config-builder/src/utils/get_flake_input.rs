use anyhow::{Context, Result};
use std::process::Command;
use super::compose_path; // Import from sibling module
use super::format_new;   // Import from sibling module
use log::debug; // Import only debug macro

pub fn get_flake_input(flake_path_str: &str, input_name: &str) -> Result<String> {
    let path_template = "path:{}";
    let path_expr = "(builtins.getFlake {}).inputs.{}.outPath";

    let composed_path = compose_path::compose_path(path_expr, path_template);
    let expr = format_new::format_new(&composed_path, flake_path_str, input_name);

    let mut command = Command::new("nix");
    command.args(&["eval", "--raw", "--extra-experimental-features", "nix-command flakes", "--expr", &expr]);

    debug!("Executing Nix command: {:?}", command);
    debug!("Working directory: {:?}
", std::env::current_dir());

    let output = command.output()
        .with_context(|| format!("Failed to execute nix eval for input '{}'", input_name))?;

    if !output.status.success() {
        anyhow::bail!(
            "Nix command failed for input '{}':\n{}\nStderr: {}",
            input_name,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8(output.stdout)?.trim().to_string();
    debug!("Nix command stdout: {}", stdout);
    Ok(stdout)
}