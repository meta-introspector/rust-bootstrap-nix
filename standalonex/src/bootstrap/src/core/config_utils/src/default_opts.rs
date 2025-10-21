use bootstrap::prelude::*;
use std::path::PathBuf;
use std::env;
use bootstrap::Config;
use bootstrap::RustOptimize;
use bootstrap::TargetSelection;
use bootstrap::CiConfig;
use std::io::IsTerminal;

pub fn default_opts() -> Config {
    let src_path = {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // Undo `src/bootstrap`
        manifest_dir.parent().unwrap().parent().unwrap().to_owned()
    };

    Config {
        bypass_bootstrap_lock: false,
        llvm_optimize: true,
        ninja_in_file: true,
        llvm_static_stdcpp: false,
        llvm_libzstd: false,
        backtrace: true,
        rust_optimize: RustOptimize::Bool(true),
        rust_optimize_tests: true,
        rust_randomize_layout: false,
        submodules: None,
        docs: true,
        docs_minification: true,
        rust_rpath: true,
        rust_strip: false,
        channel: "dev".to_string(),
        codegen_tests: true,
        rust_dist_src: true,
        rust_codegen_backends: vec!["llvm".to_owned()],
        deny_warnings: true,
        bindir: "bin".into(),
        dist_include_mingw_linker: true,
        dist_compression_profile: "fast".into(),

        stdout_is_tty: std::io::stdout().is_terminal(),
        stderr_is_tty: std::io::stderr().is_terminal(),

        // set by build.rs
        build: TargetSelection::from_user(&env::var("BUILD_TRIPLE").unwrap()),

        src: src_path.clone(),
        out: PathBuf::from("build"),

        // This is needed by codegen_ssa on macOS to ship `llvm-objcopy` aliased to
        // `rust-objcopy` to workaround bad `strip`s on macOS.
        llvm_tools_enabled: true,

        ci: CiConfig {
            channel_file: src_path.join("src/ci/channel"),
            version_file: src_path.join("src/version"),
            tools_dir: src_path.join("src/tools"),
            llvm_project_dir: src_path.join("src/llvm-project"),
            gcc_dir: src_path.join("src/gcc"),
        },

        ..Default::default()
    }
}