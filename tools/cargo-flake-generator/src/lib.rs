// src/lib.rs for cargo-flake-generator

use std::path::{Path, PathBuf};
use anyhow::{Result, Context, anyhow};
use cargo_metadata::{MetadataCommand, Metadata};
use serde::{Serialize, Deserialize};

// This struct will hold configuration for flake generation
pub struct FlakeGeneratorConfig {
    pub nixpkgs_url: String,
    pub rust_overlay_url: String,
    // Add other configuration parameters as needed, e.g., default system, etc.
}

impl Default for FlakeGeneratorConfig {
    fn default() -> Self {
        FlakeGeneratorConfig {
            nixpkgs_url: "github:NixOS/nixpkgs/nixos-23.11".to_string(), // Example default
            rust_overlay_url: "github:oxalica/rust-overlay".to_string(), // Example default
        }
    }
}

/// Generates a Nix flake for a given Cargo project.
///
/// This function takes the path to a Cargo project and generates a `flake.nix`
/// and potentially a `flake.lock` file in the project's root directory.
///
/// # Arguments
/// * `project_root` - The absolute path to the root of the Cargo project.
/// * `config` - Configuration for flake generation, including Nixpkgs and rust-overlay URLs.
///
/// # Returns
/// `Ok(())` if the flake generation is successful, otherwise an `anyhow::Error`.
pub fn generate_flake_for_crate(
    project_root: &Path,
    config: &FlakeGeneratorConfig,
) -> Result<()> {
    println!("Generating flake for crate: {}", project_root.display());

    // 1. Read Cargo.toml and Cargo.lock to get project metadata and dependencies
    let metadata = MetadataCommand::new()
        .manifest_path(project_root.join("Cargo.toml"))
        .exec()
        .context("Failed to execute cargo metadata")?;

    // Check if this is a workspace root
    if metadata.workspace_root == project_root {
        println!("Skipping flake generation for workspace root: {}", project_root.display());
        return Ok(()); // Skip generating a flake for the workspace root itself
    }

    // 2. Construct the flake.nix content based on metadata and config
    let flake_nix_content = construct_flake_nix_content(project_root, &metadata, config)?;

    // 3. Write flake.nix to the project root
    let flake_nix_path = project_root.join("flake.nix");
    std::fs::write(&flake_nix_path, flake_nix_content)
        .with_context(|| format!("Failed to write flake.nix to {}", flake_nix_path.display()))?;

    // TODO: Potentially generate flake.lock or provide instructions to the user to do so.
    // For now, we'll assume `nix flake update` will be run manually.

    Ok(())
}

/// Constructs the content of the flake.nix file.
fn construct_flake_nix_content(project_root: &Path, metadata: &Metadata, config: &FlakeGeneratorConfig) -> Result<String> {
    let root_manifest_path = project_root.join("Cargo.toml");

    let root_package = metadata.packages.iter()
        .find(|p| p.manifest_path == root_manifest_path)
        .ok_or_else(|| anyhow!("Could not find package for manifest path: {}", root_manifest_path.display()))?;

    let package_name = &root_package.name;
    let package_version = &root_package.version;

    // This is a simplified template. A real implementation would be much more complex,
    // handling dependencies, features, build scripts, etc., similar to cargo2nix output.
    let content = format!(r#"
{{
  description = "Nix flake for Rust package: {}";

  inputs = {{
    nixpkgs.url = "{}";
    rust-overlay.url = "{}";
    flake-utils.url = "github:numtide/flake-utils";
  }};

  outputs = {{ self, nixpkgs, rust-overlay, flake-utils }}:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {{
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        }};
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile {{
          rustToolchainFile = ./rust-toolchain.toml; # Assuming rust-toolchain.toml exists
        }};
      in
      {{
        packages.{} = pkgs.rustPlatform.buildRustPackage {{
          pname = "{}";
          version = "{}";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock; # Assuming Cargo.lock exists

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            # Add common build dependencies here, e.g., openssl
            # openssl
          ];

          # Environment variables for build, if needed
          # OPENSSL_LIB_DIR = "${{pkgs.lib.getLib pkgs.openssl}}/lib";
          # OPENSSL_INCLUDE_DIR = "${{pkgs.openssl.dev}}/include";
          # PKG_CONFIG_PATH = "${{pkgs.openssl.dev}}/lib/pkgconfig";
        }};
      }}
    );
}}
"#,
        package_name,
        config.nixpkgs_url,
        config.rust_overlay_url,
        package_name,
        package_name,
        package_version
    );

    Ok(content)
}