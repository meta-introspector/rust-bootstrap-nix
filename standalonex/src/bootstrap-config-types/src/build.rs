use build_helper::prelude::*;
use config_macros::define_config;
use std::collections::*;
define_config! {
    #[doc = " TOML representation of various global build decisions."] #[derive(Default)]
    struct BuildConfig { build : Option < String > = "build", src : Option < PathBuf > =
    "src", host : Option < Vec < String >> = "host", target : Option < Vec < String >> =
    "target", build_dir : Option < String > = "build-dir", cargo : Option < PathBuf > =
    "cargo", rustc : Option < PathBuf > = "rustc", rustfmt : Option < PathBuf > =
    "rustfmt", cargo_clippy : Option < PathBuf > = "cargo-clippy", docs : Option < bool >
    = "docs", compiler_docs : Option < bool > = "compiler-docs",
    library_docs_private_items : Option < bool > = "library-docs-private-items",
    docs_minification : Option < bool > = "docs-minification", submodules : Option < bool
    > = "submodules", gdb : Option < String > = "gdb", lldb : Option < String > = "lldb",
    nodejs : Option < String > = "nodejs", npm : Option < String > = "npm", python :
    Option < String > = "python", reuse : Option < String > = "reuse", locked_deps :
    Option < bool > = "locked-deps", vendor : Option < bool > = "vendor", full_bootstrap
    : Option < bool > = "full-bootstrap", bootstrap_cache_path : Option < PathBuf > =
    "bootstrap-cache-path", extended : Option < bool > = "extended", tools : Option <
    HashSet < String >> = "tools", verbose : Option < usize > = "verbose", sanitizers :
    Option < bool > = "sanitizers", profiler : Option < bool > = "profiler",
    cargo_native_static : Option < bool > = "cargo-native-static", low_priority : Option
    < bool > = "low-priority", configure_args : Option < Vec < String >> =
    "configure-args", local_rebuild : Option < bool > = "local-rebuild",
    print_step_timings : Option < bool > = "print-step-timings", print_step_rusage :
    Option < bool > = "print-step-rusage", check_stage : Option < u32 > = "check-stage",
    doc_stage : Option < u32 > = "doc-stage", build_stage : Option < u32 > =
    "build-stage", test_stage : Option < u32 > = "test-stage", install_stage : Option <
    u32 > = "install-stage", dist_stage : Option < u32 > = "dist-stage", bench_stage :
    Option < u32 > = "bench-stage", patch_binaries_for_nix : Option < bool > =
    "patch-binaries-for-nix", metrics : Option < bool > = "metrics", android_ndk : Option
    < PathBuf > = "android-ndk", optimized_compiler_builtins : Option < bool > =
    "optimized-compiler-builtins", jobs : Option < u32 > = "jobs", compiletest_diff_tool
    : Option < String > = "compiletest-diff-tool", }
}
