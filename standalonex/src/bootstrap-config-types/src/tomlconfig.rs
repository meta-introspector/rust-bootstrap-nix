use build_helper::prelude::*;
use serde::Deserialize;
/// Structure of the `config.toml` file that configuration is read from.
///
/// This structure uses `Decodable` to automatically decode a TOML configuration
/// file into this format, and then this is traversed and written into the above
/// `Config` structure.
#[derive(Deserialize, Default)]
pub struct Nix {
    pub nixpkgs_path: Option<PathBuf>,
    pub rust_overlay_path: Option<PathBuf>,
    pub rust_bootstrap_nix_path: Option<PathBuf>,
    pub configuration_nix_path: Option<PathBuf>,
    pub rust_src_flake_path: Option<PathBuf>,
}
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct TomlConfig {
    #[serde(flatten)]
    change_id: ChangeIdWrapper,
    build: Option<Build>,
    install: Option<Install>,
    llvm: Option<Llvm>,
    rust: Option<Rust>,
    target: Option<HashMap<String, TomlTarget>>,
    dist: Option<Dist>,
    ci: Option<Ci>,
    nix: Option<Nix>,
    profile: Option<String>,
    stage0_path: Option<PathBuf>,
}
