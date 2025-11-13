use crate::prelude::*;
pub fn parse_inner_toml(
    config: &mut ParsedConfig,
    flags: &LocalFlags,
    get_toml: impl Fn(&Path) -> Result<LocalTomlConfig, toml::de::Error>,
) -> LocalTomlConfig {
    let toml_path = flags
        .config
        .clone()
        .or_else(|| env::var_os("RUST_BOOTSTRAP_CONFIG").map(PathBuf::from));
    let using_default_path = toml_path.is_none();
    let mut toml_path = toml_path.unwrap_or_else(|| PathBuf::from("config.toml"));
    if using_default_path && !toml_path.exists() {
        toml_path = config.src.join(toml_path);
    }
    if !using_default_path || toml_path.exists() {
        config.config = Some(
            if cfg!(not(feature = "bootstrap-self-test")) {
                toml_path.canonicalize().unwrap()
            } else {
                toml_path.clone()
            },
        );
        get_toml(&toml_path)
            .unwrap_or_else(|e| {
                eprintln!("ERROR: Failed to parse '{}': {e}", toml_path.display());
                std::process::exit(2);
            })
    } else {
        config.config = None;
        LocalTomlConfig::default()
    }
}
