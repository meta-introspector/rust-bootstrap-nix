use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub stage: Option<String>,
    pub target: Option<String>,
    pub project_root: Option<PathBuf>,
    pub system: Option<String>,
    pub output: Option<PathBuf>,
    pub rust_bootstrap_nix_flake_ref: Option<String>,
    pub rust_src_flake_ref: Option<String>,
    pub nixpkgs_path: Option<PathBuf>,
    pub rust_overlay_path: Option<PathBuf>,
    pub rust_bootstrap_nix_path: Option<PathBuf>,
    pub configuration_nix_path: Option<PathBuf>,
    pub rust_src_flake_path: Option<PathBuf>,
    pub dry_run: Option<bool>,
}

impl AppConfig {
    pub fn merge_with_args(&mut self, args: &crate::args::Args) {
        if let Some(stage) = args.stage.clone() { self.stage = Some(stage); }
        if let Some(target) = args.target.clone() { self.target = Some(target); }
        if let Some(project_root) = args.project_root.clone() { self.project_root = Some(project_root); }
        if let Some(system) = args.system.clone() { self.system = Some(system); }
        if let Some(output) = args.output.clone() { self.output = Some(output); }
        if let Some(rust_bootstrap_nix_flake_ref) = args.rust_bootstrap_nix_flake_ref.clone() { self.rust_bootstrap_nix_flake_ref = Some(rust_bootstrap_nix_flake_ref); }
        if let Some(rust_src_flake_ref) = args.rust_src_flake_ref.clone() { self.rust_src_flake_ref = Some(rust_src_flake_ref); }
        if let Some(nixpkgs_path) = args.nixpkgs_path.clone() { self.nixpkgs_path = Some(nixpkgs_path); }
        if let Some(rust_overlay_path) = args.rust_overlay_path.clone() { self.rust_overlay_path = Some(rust_overlay_path); }
        if let Some(rust_bootstrap_nix_path) = args.rust_bootstrap_nix_path.clone() { self.rust_bootstrap_nix_path = Some(rust_bootstrap_nix_path); }
        if let Some(configuration_nix_path) = args.configuration_nix_path.clone() { self.configuration_nix_path = Some(configuration_nix_path); }
        if let Some(rust_src_flake_path) = args.rust_src_flake_path.clone() { self.rust_src_flake_path = Some(rust_src_flake_path); }
        if let Some(dry_run) = args.dry_run { self.dry_run = Some(dry_run); }
    }
}
