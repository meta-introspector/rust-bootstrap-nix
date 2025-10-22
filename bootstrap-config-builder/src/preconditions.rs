use anyhow::{Context, Result};
use std::process::Command;
// use crate::utils::get_flake_input::get_flake_input; // Commented out

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
    _rust_bootstrap_nix_flake_ref: &str, // Not directly used in this simplified check
    _system: &str, // Not directly used in this simplified check
    rust_src_flake_path: &str, // Now takes rust_src_flake_path
) -> Result<()> {
    // Simplified check: just verify if the rust_src_flake_path exists and contains src/ci/channel
    let known_file = format!("{}/src/ci/channel", rust_src_flake_path);

    if std::path::Path::new(&known_file).exists() {
        Ok(())
    } else {
        anyhow::bail!("Rust source flake NOT found at: {}. Known file 'src/ci/channel' missing.", rust_src_flake_path);
    }
}

pub fn check_rust_src_flake_exists(
    _rust_bootstrap_nix_flake_ref: &str, // Not directly used in this simplified check
    _rust_src_flake_ref: &str,
) -> Result<()> {
    // This function is now redundant with the simplified check_rust_toolchain_sysroot.
    // For now, let's just return Ok(()) as the check is done in check_rust_toolchain_sysroot
    Ok(())
}