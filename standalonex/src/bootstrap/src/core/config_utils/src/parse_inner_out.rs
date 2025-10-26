use crate::prelude::*;
pub fn parse_inner_out(config: &mut ParsedConfig) {
    if cfg!(test) {
        config.out = env::var_os("CARGO_TARGET_DIR")
            .map(|s| Path::new(&s).parent().unwrap().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("target/test_output"));
    }
}
