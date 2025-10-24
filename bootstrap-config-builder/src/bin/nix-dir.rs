use anyhow::{Context, Result};
use clap::Parser;
use std::process::Command;
use serde_json::Value;
use log::{info, debug};
use std::fs;
use serde::Deserialize;
use std::collections::HashMap;
//use crate::utils::nix_eval_utils; // Import the new module
use bootstrap_config_builder::utils::nix_eval_utils::run_nix_eval;

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default)]
    nix_packages: HashMap<String, String>,
}

/// A tool to inspect Nix flakes and their attributes.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The flake reference to inspect (e.g., "nixpkgs", "github:NixOS/nixpkgs/nixos-23.11")
    #[arg()]
    flake_ref: Option<String>,

    /// Output raw JSON from 'nix flake show' command.
    #[arg(long, default_value_t = false)]
    json: bool,

    /// Path to the config.toml file to read for Nix package information.
    #[arg(long, default_value = "config.toml")]
    config_file: String,
}

fn main() -> Result<()> {
    env_logger::init(); // Initialize the logger

    let args = Args::parse();

    
    info!("Reading config file: {}", args.config_file);
    let config_content = fs::read_to_string(&args.config_file)
        .with_context(|| format!("Failed to read config file: {}", args.config_file))?;
    let config = Some(toml::from_str::<Config>(&config_content)
        .with_context(|| format!("Failed to parse config file: {}", args.config_file))?);

    if let Some(c) = &config {
        debug!("Parsed config: {:?}", c);
    }

    if let Some(flake_ref) = args.flake_ref {
        info!("Inspecting Nix flake: {}", flake_ref);

        let mut command = Command::new("nix");
        command.args(&["flake", "show", "--json", &flake_ref]);

        debug!("Executing Nix command: {:?}", command);

        let output = command.output()
            .with_context(|| format!("Failed to execute nix flake show for '{}'", flake_ref))?;

        if !output.status.success() {
            anyhow::bail!(
                "Nix command failed for flake show '{}':\n{}\nStderr: {}",
                flake_ref,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let json_output: Value = serde_json::from_slice(&output.stdout)
            .with_context(|| "Failed to parse nix flake show JSON output")?;

        if args.json {
            println!("{}", serde_json::to_string_pretty(&json_output)?);
        } else {
            println!("Flake Attributes for {}:", flake_ref);

            if let Some(inputs) = json_output.get("inputs") {
                println!("\nInputs:");
                if let Some(inputs_obj) = inputs.as_object() {
                    for (key, _) in inputs_obj {
                        println!("  - {}", key);
                    }
                }
            }

            if let Some(outputs) = json_output.get("outputs") {
                println!("\nOutputs:");
                if let Some(outputs_obj) = outputs.as_object() {
                    for (system, system_outputs) in outputs_obj {
                        println!("  {}:", system);
                        if let Some(system_outputs_obj) = system_outputs.as_object() {
                            for (key, value) in system_outputs_obj {
                                let output_type = value.get("type").and_then(|t| t.as_str()).unwrap_or("unknown");
                                println!("    - {}: {}", key, output_type);
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(c) = &config { // Use reference to config
        println!("\nNix Packages from config.toml:");
        for (pkg_name, pkg_version) in &c.nix_packages { // Iterate over references
            match find_nix_package_store_path(pkg_name.as_str(), Some(pkg_version.as_str())) {
                Ok(Some(path)) => println!("  - {}: {}", pkg_name, path),
                Ok(None) => println!("  - {}: Not found in store (version: {})", pkg_name, pkg_version),
                Err(e) => eprintln!("  - Error finding {}: {}", pkg_name, e),
            }
        }
    }

    Ok(())
}

fn get_all_rustc_paths_from_nix_store() -> Result<Vec<String>> {
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
    Ok(stdout.lines().map(|s| s.trim().to_string()).collect())
}

fn find_nix_package_store_path(package_name: &str, version: Option<&str>) -> Result<Option<String>> {
    info!("Searching for Nix package: {}", package_name);
    if let Some(v) = version {
        info!("  Version: {}", v);
    }

    if package_name == "rustc" {
        let rustc_paths = get_all_rustc_paths_from_nix_store()?;
        if rustc_paths.is_empty() {
            return Ok(None);
        }
        // For now, just return the first one found. We'll add version parsing later.
        return Ok(Some(rustc_paths[0].clone()));
    }

    let expr = format!("with import <nixpkgs> {{}}; pkgs.{}.outPath", package_name);

    let stdout = run_nix_eval(&expr);

    match stdout {
        Ok(path) => Ok(Some(path)),
        Err(e) => {
            debug!("Nix eval failed for package '{}': {}", package_name, e);
            Ok(None)
        }
    }
}
