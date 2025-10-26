use crate::prelude::*;
pub fn parse_inner_src(
    config: &mut ParsedConfig,
    flags: &LocalFlags,
    build_src_from_toml: &Option<PathBuf>,
) {
    config.src = if let Some(src) = flags.src.clone() {
        src
    } else if let Some(src) = build_src_from_toml.clone() {
        src
    } else {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest_dir.parent().unwrap().parent().unwrap().to_owned()
    };
}
