// use anyhow::{Context, Result}; // Commented out
use std::fs;

#[allow(clippy::too_many_arguments)]
pub fn format_file(
    template_path: &str,
    system: &str,
    flake_path_str: &str,
    nixpkgs_path: &str,
    rust_overlay_path: &str,
    rust_bootstrap_nix_path: &str,
    configuration_nix_path: &str,
    rust_src_flake_path: &str,
    rust_bootstrap_nix_flake_ref: &str,
    rust_src_flake_ref: &str,
    stage: &str,
    target: &str,
) -> String {
    let template_content = fs::read_to_string(template_path)
        .expect(&format!("Failed to read template file: {}", template_path));

    // Use string replacement for each placeholder
    template_content
        .replace("{system}", system)
        .replace("{flake_path_str}", flake_path_str)
        .replace("{nixpkgs_path}", nixpkgs_path)
        .replace("{rust_overlay_path}", rust_overlay_path)
        .replace("{rust_bootstrap_nix_path}", rust_bootstrap_nix_path)
        .replace("{configuration_nix_path}", configuration_nix_path)
        .replace("{rust_src_flake_path}", rust_src_flake_path)
        .replace("{rust_bootstrap_nix_flake_ref}", rust_bootstrap_nix_flake_ref)
        .replace("{rust_src_flake_ref}", rust_src_flake_ref)
        .replace("{stage}", stage)
        .replace("{target}", target)
}
