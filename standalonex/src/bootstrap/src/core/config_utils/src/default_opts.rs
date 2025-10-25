use crate::prelude::*


//use bootstrap::prelude::*;
use std::path::PathBuf;
use crate::target_selection::TargetSelection;
use crate::local_ci_config::LocalCiConfig;
use crate::parsed_config::ParsedConfig;

use std::env;

use std::io::IsTerminal;

pub fn default_opts() -> ParsedConfig {
    let src_path = {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // Undo `src/bootstrap`
        manifest_dir.parent().unwrap().parent().unwrap().to_owned()
    };

    ParsedConfig {
        bypass_bootstrap_lock: false,
        llvm_optimize: Some(true),
        ninja_in_file: Some(true),
        llvm_static_stdcpp: Some(false),
        llvm_libzstd: Some(false),
        backtrace: Some(true),
//        rust_optimize: RustOptimize::Bool(true),
        rust_optimize_tests: Some(true),
        rust_randomize_layout: false,
        submodules: None,
        docs: Some(true),
        docs_minification: Some(true),
        rust_rpath: Some(true),
        rust_strip: Some(false),
        channel: Some("dev".to_string()),
        codegen_tests: Some(true),
        rust_dist_src: Some(true),
        rust_codegen_backends: vec!["llvm".to_owned()],
        deny_warnings: Some(true),
        bindir: Some("bin".into()),
        dist_include_mingw_linker: Some(true),
        dist_compression_profile: Some("fast".into()),

        stdout_is_tty: Some(std::io::stdout().is_terminal()),
        stderr_is_tty: Some(std::io::stderr().is_terminal()),

        // set by build.rs
        build: TargetSelection::from_user(&env::var("BUILD_TRIPLE").unwrap()),

        src: src_path.clone(),
        out: PathBuf::from("build"),

        // This is needed by codegen_ssa on macOS to ship `llvm-objcopy` aliased to
        // `rust-objcopy` to workaround bad `strip`s on macOS.
        llvm_tools_enabled: true,

        ci: Some(LocalCiConfig {
            channel_file: Some(src_path.join("src/ci/channel")),
            version_file: Some(src_path.join("src/version")),
            tools_dir: Some(src_path.join("src/tools")),
            llvm_project_dir: Some(src_path.join("src/llvm-project")),
            gcc_dir: Some(src_path.join("src/gcc")),
        }),

        ..Default::default()
    }
}
