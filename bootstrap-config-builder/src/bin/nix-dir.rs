use anyhow::{Context, Result};
use clap::Parser;
use std::process::Command;
use serde_json::Value;
use log::{info, debug};

/// A tool to inspect Nix flakes and their attributes.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The flake reference to inspect (e.g., "nixpkgs", "github:NixOS/nixpkgs/nixos-23.11")
    #[arg()]
    flake_ref: String,

    /// Output raw JSON from 'nix flake show' command.
    #[arg(long, default_value_t = false)]
    json: bool,
}

fn main() -> Result<()> {
    env_logger::init(); // Initialize the logger

    let args = Args::parse();

    info!("Inspecting Nix flake: {}", args.flake_ref);

    let mut command = Command::new("nix");
    command.args(&["flake", "show", "--json", &args.flake_ref]);

    debug!("Executing Nix command: {:?}", command);

    let output = command.output()
        .with_context(|| format!("Failed to execute nix flake show for '{}'", args.flake_ref))?;

    if !output.status.success() {
        anyhow::bail!(
            "Nix command failed for flake show '{}':\n{}\nStderr: {}",
            args.flake_ref,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let json_output: Value = serde_json::from_slice(&output.stdout)
        .with_context(|| "Failed to parse nix flake show JSON output")?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&json_output)?);
        return Ok(());
    }

    println!("Flake Attributes for {}:", args.flake_ref);

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

    Ok(())
}
