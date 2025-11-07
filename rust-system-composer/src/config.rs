use serde::Deserialize;
use std::path::Path;
use anyhow::{Result, anyhow};

#[derive(Debug, Deserialize)]
pub struct NixConfig {
    pub nixpkgs_path: String,
    pub rust_overlay_path: String,
    pub rust_bootstrap_nix_path: String,
    pub configuration_nix_path: String,
    pub rust_src_flake_path: String,
    pub rust_bootstrap_nix_flake_ref: String,
    pub rust_src_flake_ref: String,
}

#[derive(Debug, Deserialize)]
pub struct RustConfig {
    pub rustc: String,
    pub cargo: String,
    pub rustc_source: String,
    pub channel: String,
    pub rustc_version: String,
    pub rustc_host: String,
    #[serde(rename = "download-rustc")]
    pub download_rustc: bool,
    #[serde(rename = "parallel-compiler")]
    pub parallel_compiler: bool,
    #[serde(rename = "llvm-tools")]
    pub llvm_tools: bool,
    #[serde(rename = "debuginfo-level")]
    pub debuginfo_level: u8,
}

#[derive(Debug, Deserialize)]
pub struct BinsConfig {
    pub bootstrap_config_generator: String,
    pub configuration_nix: String,
    pub flake_step_manager: String,
    pub flake_template_generator: String,
    pub hf_validator: String,
    pub metrics_reporter: String,
    pub nix_dir: String,
    pub prelude_generator: String,
    pub rust_decl_splitter: String,
    pub rust_system_composer: String,
    pub expanded_code_collector: String,
    pub split_expanded_bin: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub nix: NixConfig,
    pub rust: RustConfig,
    pub bins: BinsConfig,
}

impl Config {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let config_content = std::fs::read_to_string(path)
            .map_err(|e| anyhow!("Failed to read config file at {:?}: {}", path, e))?;
        toml::from_str(&config_content)
            .map_err(|e| anyhow!("Failed to parse config file at {:?}: {}", path, e))
    }
}
