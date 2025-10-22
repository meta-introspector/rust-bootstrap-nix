use crate::parsed_config::ParsedConfig;
use crate::local_flags::LocalFlags;
use crate::local_toml_config::LocalTomlConfig;
use crate::get_toml;
use std::path::Path;
use std::path::PathBuf;
use std::env;
use std::fs;

pub fn parse_inner_toml(config: &mut ParsedConfig, flags: &LocalFlags, get_toml: impl Fn(&Path) -> Result<LocalTomlConfig, toml::de::Error>) -> LocalTomlConfig {
    // Read from `--config`, then `RUST_BOOTSTRAP_CONFIG`, then `./config.toml`, then `config.toml` in the root directory.
    let toml_path = flags
        .config
        .clone()
        .or_else(|| env::var_os("RUST_BOOTSTRAP_CONFIG").map(PathBuf::from));
    let using_default_path = toml_path.is_none();
    let mut toml_path = toml_path.unwrap_or_else(|| PathBuf::from("config.toml"));
    if using_default_path && !toml_path.exists() {
        toml_path = config.src.join(toml_path);
    }

    // Give a hard error if `--config` or `RUST_BOOTSTRAP_CONFIG` are set to a missing path,
    // but not if `config.toml` hasn't been created.
    if !using_default_path || toml_path.exists() {
        config.config = Some(if cfg!(not(feature = "bootstrap-self-test")) {
            toml_path.canonicalize().unwrap()
        } else {
            toml_path.clone()
        });
        get_toml(&toml_path).unwrap_or_else(|e| {
            eprintln!("ERROR: Failed to parse '{}': {e}", toml_path.display());
            std::process::exit(2);
        })
    } else {
        config.config = None;
        LocalTomlConfig::default()
    }
}
