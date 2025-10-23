use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the generated config.toml
    #[arg(long)]
    config_path: PathBuf,

    /// Output directory for the new flake
    #[arg(long)]
    output_dir: PathBuf,
}

#[derive(Debug, Deserialize, Default)]
struct NixConfig {
    #[serde(default)]
    nixpkgs_path: String,
    // Add other nix-related fields as needed
}

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default)]
    nix: NixConfig,
    // Add other top-level sections as needed
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Ensure output directory exists
    fs::create_dir_all(&args.output_dir)?;

    // Read config.toml content and parse it
    let config_content = fs::read_to_string(&args.config_path)?;
    let config: Config = toml::from_str(&config_content)?;

    // Extract nixpkgs_path from config.toml
    let nixpkgs_url = if config.nix.nixpkgs_path.is_empty() {
        // Fallback to the standard if not specified in config.toml
        "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify".to_string()
    } else {
        config.nix.nixpkgs_path
    };

    // Define the system architecture (can be made dynamic later)
    let system_arch = "aarch64-linux";

    // Generate flake.nix content using the extracted values
    let flake_nix_content = format!(
        r#"{{
  description = "Dynamically generated config flake";

  inputs = {{
    nixpkgs.url = "{}";
  }};

  outputs = {{ self, nixpkgs }}:
    let
      pkgs = import nixpkgs {{ system = "{}"; }};
      configTomlContent = builtins.readFile ./config.toml;
    in
    {{
      packages.{}.default = pkgs.lib.strings.toFile "config.toml" configTomlContent;
    }};
}}
"#,
        nixpkgs_url,
        system_arch,
        system_arch
    );

    // Write flake.nix to output directory
    let output_flake_nix_path = args.output_dir.join("flake.nix");
    fs::write(&output_flake_nix_path, flake_nix_content)?;

    // Copy config.toml to output directory
    let output_config_toml_path = args.output_dir.join("config.toml");
    fs::write(&output_config_toml_path, config_content)?;

    println!("Successfully generated flake in {:?}", args.output_dir);

    Ok(())
}
