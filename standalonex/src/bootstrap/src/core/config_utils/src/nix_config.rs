
use crate::parsed_config::ParsedConfig;
use crate::local_toml_config::LocalTomlConfig;
use crate::config_applicator::ConfigApplicator;

pub struct NixConfigApplicator;

impl ConfigApplicator for NixConfigApplicator {
    fn apply_to_config(&self, config: &mut ParsedConfig, toml: &LocalTomlConfig) {
        let nix_config = toml.nix.clone().unwrap_or_default();

        config.nixpkgs_path = nix_config.nixpkgs_path;
        config.rust_overlay_path = nix_config.rust_overlay_path;
        config.rust_bootstrap_nix_path = nix_config.rust_bootstrap_nix_path;
        config.configuration_nix_path = nix_config.configuration_nix_path;
        config.rust_src_flake_path = nix_config.rust_src_flake_path;
        config.rust_bootstrap_nix_flake_ref = nix_config.rust_bootstrap_nix_flake_ref;
        config.rust_src_flake_ref = nix_config.rust_src_flake_ref;
    }
}
