use anyhow::{Context, Result};
use std::path::PathBuf;
use std::pin::Pin;
use std::future::Future;
use std::boxed::Box;
use std::fmt::Debug;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub mod use_statement_types;
pub use use_statement_types::{
    GitDetails, GitInfo, GitInfoTrait,
    NixDetails, NixInfo, NixInfoTrait,
    RustDetails, RustDetailsInfo, RustDetailsInfoTrait,
    CargoDetails, CargoInfo, CargoInfoTrait,
    SynDetails, SynInfo, SynInfoTrait,
    LlvmDetails, LlvmInfo, LlvmInfoTrait,
    LinuxDetails, LinuxInfo, LinuxInfoTrait,
};

#[derive(Debug)]
pub struct RawFile(pub String, pub String);
#[derive(Clone)]
pub struct ParsedFile(pub String, pub PathBuf);
#[derive(Debug)]
pub struct UseStatements(pub Vec<String>);
#[derive(Debug)]
pub struct ClassifiedUseStatements(pub Vec<UseStatement>, pub HashMap<String, Vec<String>>);
#[derive(Debug, Clone)]
pub struct ValidatedFile(pub String, pub PathBuf);

// Functors (as a trait)
pub trait PipelineFunctor<Input: Send + 'static, Output: Send + 'static, Config> {
    fn map<'writer>(
        &'writer self,
        writer: &'writer mut (impl tokio::io::AsyncWriteExt + Unpin + Send),
        input: Input,
        _config: &'writer Option<Config>,
    ) -> Pin<Box<dyn Future<Output = Result<Output>> + Send + 'writer>>;
}

#[derive(Debug)]
pub struct UseStatement {
    pub statement: String,
    pub error: Option<String>,
    // Composed traits
    pub git_details: Option<GitDetails>,
    pub nix_details: Option<NixDetails>,
    pub rust_details: Option<RustDetails>,
    pub cargo_details: Option<CargoDetails>,
    pub syn_details: Option<SynDetails>,
    pub llvm_details: Option<LlvmDetails>,
    pub linux_details: Option<LinuxDetails>,
}

/// Information about a variable found in the AST
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableInfo {
    pub name: String,
    pub type_name: String,
    pub is_mutable: bool,
    pub scope: String, // e.g., "function", "module", "global"
}

/// Information about a function found in the AST
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub visibility: String, // e.g., "public", "private"
    pub arg_count: u32,
    pub arg_types: Vec<String>,
    pub return_type: String,
    pub is_async: bool,
    pub is_unsafe: bool,
    pub is_const: bool,
}

/// Information about an import statement found in the AST
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportInfo {
    pub path: String, // The full path of the import (e.g., "std::collections::HashMap")
    pub alias: Option<String>,
    pub is_external: bool,
    pub source_crate: Option<String>,
    pub git_source_url: Option<String>,
    pub git_branch: Option<String>,
}

/// Comprehensive AST analysis data for a Rust project
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AstStatistics {
    pub node_type_counts: HashMap<String, u32>,
    pub variable_declarations: Vec<VariableInfo>,
    pub function_definitions: Vec<FunctionInfo>,
    pub import_statements: Vec<ImportInfo>,
    // Add more fields as needed, e.g., macro invocations, struct definitions
}

// --- Content from prelude-generator/src/config_parser.rs ---
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
pub struct ModuleExportsConfig {
    pub modules: Option<Vec<String>>,
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
    #[serde(default, rename = "module_exports")]
    pub module_exports: Option<ModuleExportsConfig>,
    #[serde(default)]
    pub generated_output_dir: Option<PathBuf>,
}

pub fn read_config(config_path: &PathBuf, project_root: &PathBuf) -> Result<Config> {
    let config_content = std::fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
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

    // Resolve generated_output_dir if present and relative
    if let Some(generated_output_dir) = &mut config.generated_output_dir {
        if !generated_output_dir.is_absolute() {
            *generated_output_dir = project_root.join(&*generated_output_dir);
        }
    }

    Ok(config)
}