use super::format_file; // Import from sibling module

#[allow(clippy::too_many_arguments)]
pub fn construct_config_content(
    system: &str,
    flake_path_str: &str,
    nixpkgs_path: &str,
    rust_overlay_path: &str,
    rust_bootstrap_nix_path: &str,
    configuration_nix_path: &str,
    rust_src_flake_path: &str,
    stage: &str,
    target: &str,
    rust_bootstrap_nix_flake_ref: &str,
    rust_src_flake_ref: &str,
) -> String {
    format_file::format_file(
        "bootstrap-config-builder/src/example.toml", // Corrected path
        system,
        flake_path_str,
        nixpkgs_path,
        rust_overlay_path,
        rust_bootstrap_nix_path,
        configuration_nix_path,
        rust_src_flake_path,
        rust_bootstrap_nix_flake_ref,
        rust_src_flake_ref,
        stage,
        target
    )
}