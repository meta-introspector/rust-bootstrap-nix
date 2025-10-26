use crate::prelude::*;
pub fn check_nix_command_available() -> Result<()> {
    Command::new("nix")
        .arg("--version")
        .output()
        .with_context(|| {
            "Failed to execute 'nix --version'. Is Nix installed and in PATH?"
        })?
        .status
        .success()
        .then_some(())
        .with_context(|| {
            "'nix' command not found or failed to execute. Please install Nix."
        })
}
pub fn check_rust_toolchain_sysroot(rust_src_flake_path: &str) -> Result<()> {
    let known_file = format!("{}/src/ci/channel", rust_src_flake_path);
    if std::path::Path::new(&known_file).exists() {
        Ok(())
    } else {
        anyhow::bail!(
            "Rust source flake NOT found at: {}. Known file 'src/ci/channel' missing.",
            rust_src_flake_path
        );
    }
}
