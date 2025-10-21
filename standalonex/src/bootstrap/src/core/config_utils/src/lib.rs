// This will be the lib.rs for the new bootstrap-config-utils crate
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use bootstrap::TargetSelection;
use serde_derive::Deserialize;
pub mod default_opts;
pub mod get_builder_toml;
pub mod get_toml;
pub mod parse;
pub mod parse_inner;
pub mod parse_inner_flags;
pub mod parse_inner_src;
pub mod parse_inner_out;
pub mod parse_inner_stage0;
pub mod parse_inner_toml;
pub mod parse_inner_build;
pub mod dry_run;
pub mod try_run;
pub mod ci_config;
pub mod build_config;
pub mod install_config;

pub trait ConfigApplicator {
    fn apply_to_config(&self, config: &mut ParsedConfig, toml: &LocalTomlConfig);
}

#[derive(Debug, Default)]
pub struct ParsedConfig {
    pub channel_file: Option<std::path::PathBuf>,
    pub version_file: Option<std::path::PathBuf>,
    pub tools_dir: Option<std::path::PathBuf>,
    pub llvm_project_dir: Option<std::path::PathBuf>,
    pub gcc_dir: Option<std::path::PathBuf>,
    pub change_id: Option<String>,
    pub jobs: Option<usize>,
    pub build_triple: Option<String>,
    pub out_dir: Option<std::path::PathBuf>,
    pub initial_cargo_clippy: Option<bool>,
    pub initial_rustc: Option<std::path::PathBuf>,
    pub initial_cargo: Option<std::path::PathBuf>,
    pub dry_run: bool,
    pub hosts: Vec<String>,
    pub targets: Vec<String>,
    pub target_config: std::collections::HashMap<TargetSelection, LocalTargetConfig>,
    pub nodejs: Option<std::path::PathBuf>,
    pub npm: Option<std::path::PathBuf>,
    pub gdb: Option<std::path::PathBuf>,
    pub lldb: Option<std::path::PathBuf>,
    pub python: Option<std::path::PathBuf>,
    pub reuse: Option<std::path::PathBuf>,
    pub submodules: Option<bool>,
    pub android_ndk: Option<std::path::PathBuf>,
    pub bootstrap_cache_path: Option<std::path::PathBuf>,
    pub low_priority: Option<bool>,
    pub compiler_docs: Option<bool>,
    pub library_docs_private_items: Option<bool>,
    pub docs_minification: Option<bool>,
    pub docs: Option<bool>,
    pub locked_deps: Option<bool>,
    pub vendor: Option<bool>,
    pub full_bootstrap: Option<bool>,
    pub extended: Option<bool>,
    pub tools: Option<Vec<String>>,
    pub verbose: Option<usize>,
    pub sanitizers: Option<bool>,
    pub profiler: Option<bool>,
    pub cargo_native_static: Option<bool>,
    pub configure_args: Option<Vec<String>>,
    pub local_rebuild: Option<bool>,
    pub print_step_timings: Option<bool>,
    pub print_step_rusage: Option<bool>,
    pub patch_binaries_for_nix: Option<bool>,
    pub verbose_tests: bool,
    pub prefix: Option<std::path::PathBuf>,
    pub sysconfdir: Option<std::path::PathBuf>,
    pub datadir: Option<std::path::PathBuf>,
    pub docdir: Option<std::path::PathBuf>,
    pub bindir: Option<std::path::PathBuf>,
    pub libdir: Option<std::path::PathBuf>,
    pub mandir: Option<std::path::PathBuf>,
    pub llvm_assertions: bool,
    pub llvm_tests: bool,
    pub llvm_enzyme: bool,
    pub llvm_offload: bool,
    pub llvm_plugins: bool,
    pub rust_optimize: Option<String>, // Will be converted to RustOptimize enum later
    pub omit_git_hash: bool,
    pub rust_new_symbol_mangling: Option<bool>,
    pub rust_optimize_tests: Option<bool>,
    pub rust_rpath: Option<bool>,
    pub rust_strip: Option<bool>,
    pub rust_frame_pointers: Option<bool>,
    pub rust_stack_protector: Option<bool>,
    pub jemalloc: Option<bool>,
    pub test_compare_mode: Option<String>,
    pub backtrace: Option<bool>,
    pub description: Option<String>,
    pub rust_dist_src: Option<bool>,
    pub verbose_tests_flag: Option<bool>, // Renamed to avoid conflict with config.verbose_tests
    pub incremental: bool,
    pub lld_mode: Option<String>, // Will be converted to LldMode enum later
    pub llvm_bitcode_linker_enabled: Option<bool>,
    pub rust_randomize_layout: bool,
    pub llvm_tools_enabled: bool,
    pub llvm_enzyme_flag: Option<bool>, // Renamed to avoid conflict with config.llvm_enzyme
    pub rustc_default_linker: Option<String>,
    pub musl_root: Option<std::path::PathBuf>,
    pub save_toolstates: Option<std::path::PathBuf>,
    pub deny_warnings: Option<bool>,
    pub backtrace_on_ice: Option<bool>,
    pub rust_verify_llvm_ir: Option<bool>,
    pub rust_thin_lto_import_instr_limit: Option<u32>,
    pub rust_remap_debuginfo: Option<bool>,
    pub control_flow_guard: Option<bool>,
    pub ehcont_guard: Option<bool>,
    pub llvm_libunwind_default: Option<String>,
    pub rust_codegen_backends: Vec<String>,
    pub rust_codegen_units: Option<usize>,
    pub rust_codegen_units_std: Option<usize>,
    pub rust_profile_use: Option<bool>,
    pub rust_profile_generate: Option<bool>,
    pub rust_lto: Option<String>, // Will be converted to RustcLto enum later
    pub rust_validate_mir_opts: Option<String>,
    pub reproducible_artifacts: bool,
    pub download_rustc_commit: Option<String>,
    pub llvm_from_ci: bool,
    pub llvm_optimize: Option<bool>,
    pub llvm_thin_lto: Option<bool>,
    pub llvm_release_debuginfo: Option<bool>,
    pub llvm_static_stdcpp: Option<bool>,
    pub llvm_libzstd: Option<bool>,
    pub llvm_link_shared: Option<bool>,
    pub llvm_targets: Vec<String>,
    pub llvm_experimental_targets: Vec<String>,
    pub llvm_link_jobs: Option<usize>,
    pub llvm_version_suffix: Option<String>,
    pub llvm_clang_cl: Option<String>,
    pub llvm_enable_projects: Vec<String>,
    pub llvm_cflags: Option<String>,
    pub llvm_cxxflags: Option<String>,
    pub llvm_ldflags: Option<String>,
    pub llvm_use_libcxx: Option<bool>,
    pub llvm_use_linker: Option<String>,
    pub llvm_allow_old_toolchain: bool,
    pub llvm_polly: bool,
    pub llvm_clang: bool,
    pub llvm_enable_warnings: bool,
    pub ccache: Option<String>,
    pub ninja_in_file: Option<bool>,
    pub llvm_build_config: Option<String>,
    pub dist_sign_folder: Option<std::path::PathBuf>,
    pub dist_upload_addr: Option<String>,
    pub dist_compression_formats: Option<Vec<String>>,
    pub dist_compression_profile: Option<String>,
    pub dist_include_mingw_linker: Option<bool>,
    pub dist_vendor: bool,
    pub initial_rustfmt: Option<String>, // Will be converted to RustfmtState enum later
    pub lld_enabled: bool,
    pub rust_std_features: std::collections::BTreeSet<String>,
    pub rustc_debug_assertions: bool,
    pub std_debug_assertions: bool,
    pub rust_overflow_checks: bool,
    pub rust_overflow_checks_std: bool,
    pub rust_debug_logging: bool,
    pub rust_debuginfo_level_rustc: Option<String>, // Will be converted to DebuginfoLevel enum later
    pub rust_debuginfo_level_std: Option<String>, // Will be converted to DebuginfoLevel enum later
    pub rust_debuginfo_level_tools: Option<String>, // Will be converted to DebuginfoLevel enum later
    pub rust_debuginfo_level_tests: Option<String>, // Will be converted to DebuginfoLevel enum later
    pub optimized_compiler_builtins: bool,
    pub compiletest_diff_tool: Option<String>,
    pub stage: usize,
    pub cmd: Option<String>, // Will be converted to Subcommand enum later
}

#[derive(Debug, Default)]
pub struct LocalFlags {
    pub set: Vec<String>,
    pub jobs: Option<usize>,
    pub build_dir: Option<std::path::PathBuf>,
    pub skip_stage0_validation: bool,
    pub host: Option<Vec<String>>,
    pub target: Option<Vec<String>>,
    pub warnings: Option<String>, // Will be converted to Warnings enum later
    pub rust_profile_use: Option<bool>,
    pub rust_profile_generate: Option<bool>,
    pub reproducible_artifact: bool,
    pub verbose: usize,
    pub stage: Option<usize>,
    pub subcommand: Option<String>,
    pub dry_run: bool,
    pub incremental: bool,
}


#[derive(Debug, Default, Deserialize)]
#[derive(Clone)]
pub struct LocalCiConfig {
    pub channel_file: Option<std::path::PathBuf>,
    pub version_file: Option<std::path::PathBuf>,
    pub tools_dir: Option<std::path::PathBuf>,
    pub llvm_project_dir: Option<std::path::PathBuf>,
    pub gcc_dir: Option<std::path::PathBuf>,
}


#[derive(Debug, Default, Deserialize)]
#[derive(Clone)]
pub struct LocalBuild {
    pub build: Option<String>,
    pub host: Option<Vec<String>>,
    pub target: Option<Vec<String>>,
    pub build_dir: Option<std::path::PathBuf>,
    pub cargo: Option<std::path::PathBuf>,
    pub rustc: Option<std::path::PathBuf>,
    pub rustfmt: Option<std::path::PathBuf>,
    pub cargo_clippy: Option<bool>,
    pub docs: Option<bool>,
    pub compiler_docs: Option<bool>,
    pub library_docs_private_items: Option<bool>,
    pub docs_minification: Option<bool>,
    pub submodules: Option<bool>,
    pub gdb: Option<std::path::PathBuf>,
    pub lldb: Option<std::path::PathBuf>,
    pub nodejs: Option<std::path::PathBuf>,
    pub npm: Option<std::path::PathBuf>,
    pub python: Option<std::path::PathBuf>,
    pub reuse: Option<std::path::PathBuf>,
    pub locked_deps: Option<bool>,
    pub vendor: Option<bool>,
    pub full_bootstrap: Option<bool>,
    pub bootstrap_cache_path: Option<std::path::PathBuf>,
    pub extended: Option<bool>,
    pub tools: Option<Vec<String>>,
    pub verbose: Option<usize>,
    pub sanitizers: Option<bool>,
    pub profiler: Option<bool>,
    pub cargo_native_static: Option<bool>,
    pub low_priority: Option<bool>,
    pub configure_args: Option<Vec<String>>,
    pub local_rebuild: Option<bool>,
    pub print_step_timings: Option<bool>,
    pub print_step_rusage: Option<bool>,
    pub check_stage: Option<usize>,
    pub doc_stage: Option<usize>,
    pub build_stage: Option<usize>,
    pub test_stage: Option<usize>,
    pub install_stage: Option<usize>,
    pub dist_stage: Option<usize>,
    pub bench_stage: Option<usize>,
    pub patch_binaries_for_nix: Option<bool>,
    pub metrics: Option<bool>,
    pub android_ndk: Option<std::path::PathBuf>,
    pub optimized_compiler_builtins: Option<bool>,
    pub jobs: Option<usize>,
    pub compiletest_diff_tool: Option<String>,
    pub src: Option<std::path::PathBuf>,
}


#[derive(Debug, Default, Deserialize)]
pub struct LocalLlvm {
    pub optimize: Option<bool>,
    pub thin_lto: Option<bool>,
    pub release_debuginfo: Option<bool>,
    pub assertions: Option<bool>,
    pub tests: Option<bool>,
    pub enzyme: Option<bool>,
    pub plugins: Option<bool>,
    pub ccache: Option<String>,
    pub static_libstdcpp: Option<bool>,
    pub libzstd: Option<bool>,
    pub ninja: Option<bool>,
    pub targets: Option<Vec<String>>,
    pub experimental_targets: Option<Vec<String>>,
    pub link_jobs: Option<usize>,
    pub link_shared: Option<bool>,
    pub version_suffix: Option<String>,
    pub clang_cl: Option<String>,
    pub cflags: Option<String>,
    pub cxxflags: Option<String>,
    pub ldflags: Option<String>,
    pub use_libcxx: Option<bool>,
    pub use_linker: Option<String>,
    pub allow_old_toolchain: Option<bool>,
    pub offload: Option<bool>,
    pub polly: Option<bool>,
    pub clang: Option<bool>,
    pub enable_warnings: Option<bool>,
    pub download_ci_llvm: Option<bool>,
    pub build_config: Option<String>,
    pub enable_projects: Option<Vec<String>>,
}


#[derive(Debug, Default, Deserialize)]
pub struct LocalRust {
    pub optimize: Option<String>,
    pub debug: Option<bool>,
    pub codegen_units: Option<usize>,
    pub codegen_units_std: Option<usize>,
    pub rustc_debug_assertions: Option<bool>,
    pub std_debug_assertions: Option<bool>,
    pub overflow_checks: Option<bool>,
    pub overflow_checks_std: Option<bool>,
    pub debug_logging: Option<bool>,
    pub debuginfo_level: Option<String>,
    pub debuginfo_level_rustc: Option<String>,
    pub debuginfo_level_std: Option<String>,
    pub debuginfo_level_tools: Option<String>,
    pub debuginfo_level_tests: Option<String>,
    pub backtrace: Option<bool>,
    pub incremental: Option<bool>,
    pub parallel_compiler: Option<bool>,
    pub randomize_layout: Option<bool>,
    pub default_linker: Option<String>,
    pub channel: Option<String>,
    pub description: Option<String>,
    pub musl_root: Option<std::path::PathBuf>,
    pub rpath: Option<bool>,
    pub verbose_tests: Option<bool>,
    pub optimize_tests: Option<bool>,
    pub codegen_tests: Option<bool>,
    pub omit_git_hash: Option<bool>,
    pub dist_src: Option<bool>,
    pub save_toolstates: Option<std::path::PathBuf>,
    pub codegen_backends: Option<Vec<String>>,
    pub lld: Option<bool>,
    pub llvm_tools: Option<bool>,
    pub llvm_bitcode_linker: Option<bool>,
    pub deny_warnings: Option<bool>,
    pub backtrace_on_ice: Option<bool>,
    pub verify_llvm_ir: Option<bool>,
    pub thin_lto_import_instr_limit: Option<u32>,
    pub remap_debuginfo: Option<bool>,
    pub jemalloc: Option<bool>,
    pub test_compare_mode: Option<String>,
    pub llvm_libunwind: Option<String>,
    pub control_flow_guard: Option<bool>,
    pub ehcont_guard: Option<bool>,
    pub new_symbol_mangling: Option<bool>,
    pub profile_generate: Option<bool>,
    pub profile_use: Option<bool>,
    pub download_rustc: Option<bool>,
    pub lto: Option<String>,
    pub validate_mir_opts: Option<String>,
    pub frame_pointers: Option<bool>,
    pub stack_protector: Option<bool>,
    pub strip: Option<bool>,
    pub lld_mode: Option<String>,
    pub std_features: Option<std::collections::BTreeSet<String>>,
}


#[derive(Debug, Default, Deserialize)]
pub struct LocalTargetConfig {
    pub llvm_config: Option<std::path::PathBuf>,
    pub llvm_has_rust_patches: Option<bool>,
    pub llvm_filecheck: Option<std::path::PathBuf>,
    pub llvm_libunwind: Option<String>,
    pub no_std: Option<bool>,
    pub cc: Option<std::path::PathBuf>,
    pub cxx: Option<std::path::PathBuf>,
    pub ar: Option<std::path::PathBuf>,
    pub ranlib: Option<std::path::PathBuf>,
    pub linker: Option<std::path::PathBuf>,
    pub crt_static: Option<bool>,
    pub musl_root: Option<std::path::PathBuf>,
    pub musl_libdir: Option<std::path::PathBuf>,
    pub wasi_root: Option<std::path::PathBuf>,
    pub qemu_rootfs: Option<std::path::PathBuf>,
    pub runner: Option<Vec<String>>,
    pub sanitizers: Option<bool>,
    pub profiler: Option<bool>,
    pub rpath: Option<bool>,
    pub codegen_backends: Option<Vec<String>>,
    pub split_debuginfo: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct LocalTomlConfig {
    pub ci: Option<LocalCiConfig>,
    pub build: Option<LocalBuild>,
    pub llvm: Option<LocalLlvm>,
    pub rust: Option<LocalRust>,
    pub target: Option<std::collections::HashMap<String, LocalTargetConfig>>,
    pub install: Option<install_config::Install>,
    // ... other fields will go here
}