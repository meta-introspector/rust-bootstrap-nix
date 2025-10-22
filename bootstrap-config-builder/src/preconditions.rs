use anyhow::{Context, Result};
use std::process::Command;
use crate::utils; // Import the utils module

pub fn check_nix_command_available() -> Result<()> {
    Command::new("nix")
        .arg("--version")
        .output()
        .with_context(|| "Failed to execute 'nix --version'. Is Nix installed and in PATH?")?
        .status
        .success()
        .then_some(())
        .with_context(|| "'nix' command not found or failed to execute. Please install Nix.")
}

pub fn check_rust_toolchain_sysroot(
    rust_bootstrap_nix_flake_ref: &str,
    system: &str,
    rust_overlay_ref: &str,
) -> Result<()> {
    let expr = format!(
        r#" 
        let
          standalonexFlake = builtins.getFlake "{}";
          pkgs = import standalonexFlake.inputs.nixpkgs {{
            system = "{}";
            overlays = [ (builtins.getFlake "{}").overlays.default ];
          }};
        in
        pkgs.rustPlatform.rustLibSrc
        "#,
        rust_bootstrap_nix_flake_ref,
        system,
        rust_overlay_ref
    );

    let rust_toolchain_path = Command::new("nix")
        .args(&["eval", "--raw", "--extra-experimental-features", "nix-command flakes", "--expr", &expr])
        .output()
        .with_context(|| "Failed to execute nix eval for rust toolchain sysroot")?;

    if !rust_toolchain_path.status.success() {
        anyhow::bail!(
            "Nix command failed for rust toolchain sysroot:\n{}",
            String::from_utf8_lossy(&rust_toolchain_path.stderr)
        );
    }

    let path_str = String::from_utf8(rust_toolchain_path.stdout)?.trim().to_string();
    let full_path = format!("{}/lib/rustlib/src/rust", path_str);

    if std::path::Path::new(&full_path).exists() {
        Ok(())
    } else {
        anyhow::bail!("Rust toolchain sysroot NOT found at: {}", full_path);
    }
}

pub fn check_rust_src_flake_exists(
    rust_bootstrap_nix_flake_ref: &str,
    rust_src_flake_ref: &str,
) -> Result<()> {
    let expr = format!(
        r#" 
        let
          standalonexFlake = builtins.getFlake "{}";
        in
        (builtins.getFlake "{}").outPath
        "#,
        rust_bootstrap_nix_flake_ref,
        rust_src_flake_ref
    );

    let rust_src_flake_path = Command::new("nix")
        .args(&["eval", "--raw", "--extra-experimental-features", "nix-command flakes", "--expr", &expr])
        .output()
        .with_context(|| "Failed to execute nix eval for rust source flake")?;

    if !rust_src_flake_path.status.success() {
        anyhow::bail!(
            "Nix command failed for rust source flake:\n{}",
            String::from_utf8_lossy(&rust_src_flake_path.stderr)
        );
    }

    let path_str = String::from_utf8(rust_src_flake_path.stdout)?.trim().to_string();
    let known_file = format!("{}/src/ci/channel", path_str);

    if std::path::Path::new(&known_file).exists() {
        Ok(())
    } else {
        anyhow::bail!("Known file 'src/ci/channel' NOT found within Rust source flake. Path might be incorrect or incomplete: {}", known_file);
    }
}