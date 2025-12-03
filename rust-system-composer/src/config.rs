use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize}; // Added Serialize for consistency, if needed later
use std::path::{Path, PathBuf};

// Import necessary types from the canonical config
use standalonex::bootstrap::core::config_standalone::Config as CanonicalConfig;
use standalonex::bootstrap::core::config_standalone::TargetSelection;
use standalonex::bootstrap::core::config::debug_info_level::DebuginfoLevel;
use standalonex::bootstrap::build_helper::channel; // For GitInfo, if needed

// The new Config struct will be a direct subset/mirror of CanonicalConfig fields relevant to rust-system-composer
#[derive(Debug, Deserialize, Serialize, Default, Clone)] // Added Serialize, Default, Clone
#[serde(default)] // Deserialize unknown fields as their default values
pub struct Config {
    // From NixConfig
    #[serde(rename = "nixpkgs-path")]
    pub nixpkgs_path: Option<PathBuf>, // Canonical uses Option<PathBuf>
    #[serde(rename = "rust-overlay-path")]
    pub rust_overlay_path: Option<PathBuf>,
    #[serde(rename = "rust-bootstrap-nix-path")]
    pub rust_bootstrap_nix_path: Option<PathBuf>,
    #[serde(rename = "configuration-nix-path")]
    pub configuration_nix_path: Option<PathBuf>,
    #[serde(rename = "rust-src-flake-path")]
    pub rust_src_flake_path: Option<PathBuf>,
    #[serde(rename = "rust-bootstrap-nix-flake-ref")]
    pub rust_bootstrap_nix_flake_ref: Option<String>,
    #[serde(rename = "rust-src-flake-ref")]
    pub rust_src_flake_ref: Option<String>,

    // From RustConfig
    pub rustc: Option<PathBuf>, // Canonical uses initial_rustc: PathBuf, but we'll use Option<PathBuf> for flexibility
    pub cargo: Option<PathBuf>, // Canonical uses initial_cargo: PathBuf, but we'll use Option<PathBuf> for flexibility
    #[serde(rename = "rustc-source")]
    pub rustc_source: Option<PathBuf>, // Canonical has rustc_source: Option<PathBuf>
    pub channel: Option<String>, // Canonical has channel: String
    #[serde(rename = "rustc-version")]
    pub rustc_version: Option<String>, // Canonical uses download_rustc_commit: Option<String>
    #[serde(rename = "rustc-host")]
    pub rustc_host: Option<String>, // Canonical uses hosts: Vec<TargetSelection>
    #[serde(rename = "download-rustc")]
    pub rust_download_rustc: Option<bool>, // Canonical has download_rustc_commit: Option<String> (boolean implies setting a commit)
    #[serde(rename = "parallel-compiler")]
    pub rust_parallel_compiler: Option<bool>, // Canonical rust_parallel_compiler: bool
    #[serde(rename = "llvm-tools")]
    pub rust_llvm_tools: Option<bool>, // Canonical llvm_tools_enabled: bool
    #[serde(rename = "debuginfo-level")]
    pub rust_debuginfo_level: Option<u8>, // Canonical rust_debuginfo_level_rustc: DebuginfoLevel

    // From BinsConfig - these are paths to binaries, map to PathBuf
    #[serde(rename = "bootstrap-config-generator")]
    pub bootstrap_config_generator: Option<PathBuf>,
    #[serde(rename = "configuration-nix")]
    pub configuration_nix_bin: Option<PathBuf>, // Avoid clash with configuration_nix_path
    #[serde(rename = "flake-step-manager")]
    pub flake_step_manager: Option<PathBuf>,
    #[serde(rename = "flake-template-generator")]
    pub flake_template_generator: Option<PathBuf>,
    #[serde(rename = "hf-validator")]
    pub hf_validator: Option<PathBuf>,
    #[serde(rename = "metrics-reporter")]
    pub metrics_reporter: Option<PathBuf>,
    #[serde(rename = "nix-dir")]
    pub nix_dir_bin: Option<PathBuf>, // Avoid clash
    #[serde(rename = "prelude-generator")]
    pub prelude_generator: Option<PathBuf>,
    #[serde(rename = "rust-decl-splitter")]
    pub rust_decl_splitter: Option<PathBuf>,
    #[serde(rename = "rust-system-composer")]
    pub rust_system_composer_bin: Option<PathBuf>, // Avoid clash
    #[serde(rename = "expanded-code-collector")]
    pub expanded_code_collector: Option<PathBuf>,
    #[serde(rename = "split-expanded-bin")]
    pub split_expanded_bin: Option<PathBuf>,

    // From PathsConfig
    #[serde(rename = "generated-declarations-root")]
    pub generated_declarations_root: Option<PathBuf>, // Canonical out: PathBuf
    #[serde(rename = "default-vendor-dir")]
    pub default_vendor_dir: Option<PathBuf>,
    #[serde(rename = "code-graph-output-path")]
    pub code_graph_output_path: Option<PathBuf>,
    #[serde(rename = "command-report-output-path")]
    pub command_report_output_path: Option<PathBuf>,
    #[serde(rename = "exclude-paths")]
    pub exclude_paths: Option<Vec<PathBuf>>,
}

impl Config {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let config_content = std::fs::read_to_string(path)
            .map_err(|e| anyhow!("Failed to read config file at {:?}: {}", path, e))?;
        println!("--- Config File Content (from rust-system-composer) ---");
        println!("{}", config_content);
        println!("-------------------------------------------------------");
        let config: Self = toml::from_str(&config_content)
            .map_err(|e| anyhow!("Failed to parse config file at {:?}: {}", path, e))?;

        Ok(config)
    }

    pub fn to_canonical_config(&self) -> CanonicalConfig {
        let mut canonical_config = CanonicalConfig::default();

        // Map Nix-related fields
        canonical_config.nixpkgs_path = self.nixpkgs_path.clone();
        canonical_config.rust_overlay_path = self.rust_overlay_path.clone();
        canonical_config.rust_bootstrap_nix_path = self.rust_bootstrap_nix_path.clone();
        canonical_config.configuration_nix_path = self.configuration_nix_path.clone();
        canonical_config.rust_src_flake_path = self.rust_src_flake_path.clone();
        canonical_config.rust_info.flake_ref = self.rust_bootstrap_nix_flake_ref.clone().unwrap_or_default();
        canonical_config.cargo_info.flake_ref = self.rust_src_flake_ref.clone().unwrap_or_default();

        // Map Rust-related fields
        canonical_config.initial_rustc = self.rustc.clone().unwrap_or_default();
        canonical_config.initial_cargo = self.cargo.clone().unwrap_or_default();
        canonical_config.rustc_source = self.rustc_source.clone();
        canonical_config.channel = self.channel.clone().unwrap_or_default();
        canonical_config.download_rustc_commit = self.rustc_version.clone();
        
        if let Some(host_str) = &self.rustc_host {
            canonical_config.hosts.push(TargetSelection::from(host_str.as_str()));
        }

        canonical_config.rust_download_rustc = self.rust_download_rustc.unwrap_or_default();
        canonical_config.rust_parallel_compiler = self.rust_parallel_compiler.unwrap_or_default();
        canonical_config.llvm_tools_enabled = self.rust_llvm_tools.unwrap_or_default();
        // CanonicalConfig uses DebuginfoLevel enum, convert u8
        canonical_config.rust_debuginfo_level_rustc = self.rust_debuginfo_level.map(|l| DebuginfoLevel::from(l)).unwrap_or_default();

        // Map PathsConfig fields
        canonical_config.out = self.generated_declarations_root.clone().unwrap_or_default();
        // There is no direct default_vendor_dir in CanonicalConfig, it's typically handled as a build argument
        // canonical_config.default_vendor_dir = self.default_vendor_dir.clone();
        // CanonicalConfig.paths is Vec<PathBuf>, exclude_paths can be mapped here
        if let Some(exclude_paths) = &self.exclude_paths {
            canonical_config.skip = exclude_paths.clone();
        }
        
        // BinsConfig fields are tool paths, which CanonicalConfig might manage as part of `tools` set or specific paths
        // This mapping is more complex and might require adding new fields to CanonicalConfig or handling dynamically.
        // For now, these binary paths are not directly mapped to CanonicalConfig.

        canonical_config
    }
}
