use bootstrap::Config;
use std::path::Path;
use std::env;

pub fn parse_inner_out(config: &mut Config) {
    if cfg!(test) {
        // Use the build directory of the original x.py invocation, so that we can set `initial_rustc` properly.
        config.out = Path::new(
            &env::var_os("CARGO_TARGET_DIR").expect("cargo test directly is not supported"),
        )
        .parent()
        .unwrap()
        .to_path_buf();
    }
}
