use crate::prelude::*


use serde::{Serialize, Deserialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct NixPaths {
    rustc_path: String,
    cargo_path: String,
    // Add other paths as needed
}

fn get_command_path(command_name: &str) -> Result<String, String> {
    let output = Command::new("which")
        .arg(command_name)
        .output()
        .map_err(|e| format!("Failed to execute 'which {}': {}", command_name, e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(format!("Command '{}' not found in PATH", command_name))
    }
}

fn main() -> Result<(), String> {
    println!("Stage 1 Booster Bootstrap: Assessing configuration...");

    let rustc_path = get_command_path("rustc")?;
    let cargo_path = get_command_path("cargo")?;

    let nix_paths = NixPaths {
        rustc_path,
        cargo_path,
    };

    let json_output = serde_json::to_string_pretty(&nix_paths)
        .map_err(|e| format!("Failed to serialize NixPaths to JSON: {}", e))?;

    println!("{}", json_output);

    Ok(())
}
