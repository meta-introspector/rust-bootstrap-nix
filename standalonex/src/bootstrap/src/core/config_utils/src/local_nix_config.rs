use crate::prelude::*


use serde_derive::Deserialize;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct LocalNixConfig {
    pub nixpkgs_path: Option<String>,
    pub rust_overlay_path: Option<String>,
    pub rust_bootstrap_nix_path: Option<String>,
    pub configuration_nix_path: Option<String>,
    pub rust_src_flake_path: Option<String>,
    pub rust_bootstrap_nix_flake_ref: Option<String>,
    pub rust_src_flake_ref: Option<String>,
}
