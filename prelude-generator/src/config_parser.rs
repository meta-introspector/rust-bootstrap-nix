use serde::Deserialize;
use std::path::PathBuf;
use anyhow::{Result, Context};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct NixConfig {
    pub nixpkgs_path: Option<String>,
    pub rust_overlay_path: Option<String>,
    pub rust_bootstrap_nix_path: Option<String>,
    pub configuration_nix_path: Option<String>,
    pub rust_src_flake_path: Option<PathBuf>,
    pub rust_bootstrap_nix_flake_ref: Option<String>,
    pub rust_src_flake_ref: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RustConfig {
    pub rustc: Option<String>,
    pub cargo: Option<String>,
    pub channel: Option<String>,
    #[serde(rename = "download-rustc")]
    pub download_rustc: Option<bool>,
    #[serde(rename = "parallel-compiler")]
    pub parallel_compiler: Option<bool>,
    #[serde(rename = "llvm-tools")]
    pub llvm_tools: Option<bool>,
    #[serde(rename = "debuginfo-level")]
    pub debuginfo_level: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BuildConfig {
    pub stage: Option<String>,
    pub target: Option<String>,
    #[serde(rename = "patch-binaries-for-nix")]
    pub patch_binaries_for_nix: Option<bool>,
    pub vendor: Option<bool>,
    #[serde(rename = "build-dir")]
    pub build_dir: Option<String>,
    pub jobs: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EnvConfig {
    #[serde(rename = "HOME")]
    pub home: Option<String>,
    #[serde(rename = "CARGO_HOME")]
    pub cargo_home: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InstallConfig {
    pub prefix: Option<String>,
    pub sysconfdir: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DistConfig {
    #[serde(rename = "sign-folder")]
    pub sign_folder: Option<String>,
    #[serde(rename = "upload-addr")]
    pub upload_addr: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LlvmConfig {
    #[serde(rename = "download-ci-llvm")]
    pub download_ci_llvm: Option<bool>,
    pub ninja: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChangeIdConfig {
    pub id: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BinsConfig {
    #[serde(flatten)]
    pub paths: HashMap<String, PathBuf>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub nix: Option<NixConfig>,
    #[serde(default)]
    pub rust: Option<RustConfig>,
    #[serde(default)]
    pub build: Option<BuildConfig>,
    #[serde(default)]
    pub env: Option<EnvConfig>,
    #[serde(default)]
    pub install: Option<InstallConfig>,
    #[serde(default)]
    pub dist: Option<DistConfig>,
    #[serde(default)]
    pub llvm: Option<LlvmConfig>,
    #[serde(default, rename = "change-id")]
    pub change_id: Option<ChangeIdConfig>,
    #[serde(default)]
    pub bins: Option<BinsConfig>,
}

pub fn read_config(config_path: &PathBuf, project_root: &PathBuf) -> Result<Config> {
    let config_content = std::fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
    println!("Config content:\n{}", config_content);
    let mut config: Config = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;

    // Resolve paths in bins section
    if let Some(bins_config) = &mut config.bins {
        for (_, path) in bins_config.paths.iter_mut() {
            if !path.is_absolute() {
                *path = project_root.join(&path);
            }
        }
    }

    Ok(config)
}