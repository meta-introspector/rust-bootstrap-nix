use crate::prelude::*;
/// Structure of the `config.toml` file that configuration is read from.
///
/// This structure uses `Decodable` to automatically decode a TOML configuration
/// file into this format, and then this is traversed and written into the above
/// `Config` structure.
#[derive(Deserialize, Default)]
pub(crate) struct Nix {
    nixpkgs_path: Option<PathBuf>,
    rust_overlay_path: Option<PathBuf>,
    rust_bootstrap_nix_path: Option<PathBuf>,
    configuration_nix_path: Option<PathBuf>,
    rust_src_flake_path: Option<PathBuf>,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub(crate) struct TomlConfig {
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

