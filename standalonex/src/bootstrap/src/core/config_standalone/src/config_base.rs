use std::process::Command;
use std::str::FromStr;

use crate::core::config::ci::CiConfig;
use crate::core::config::debug_info_level::DebuginfoLevel;
/// Global configuration for the entire build and/or bootstrap.
///
/// This structure is parsed from `config.toml`, and some of the fields are inferred from `git` or build-time parameters.
///
/// Note that this structure is not decoded directly into, but rather it is
/// filled out from the decoded forms of the structs below. For documentation
/// each field, see the corresponding fields in
/// `config.example.toml`.
#[derive(Default, Clone)]
use crate::core::config::flags::{Color, Flags, Warnings};
use crate::core::config::lld_mode::LldMode;
use crate::core::config::llvm_lib_unwind::LlvmLibunwind;
use crate::core::config::rust_optimize::RustOptimize;
use crate::core::config::rustclto::RustcLto;
use crate::core::config::rustfmt::RustfmtState;
use crate::core::config::subcommand::Subcommand;
use crate::core::config::target::Target;
use crate::core::config_utils::parsed_config::ParsedConfig;
use crate::prelude::*;
pub struct Config {
    pub change_id: Option<usize>,
    pub rustc_source: Option<PathBuf>,
    pub bypass_bootstrap_lock: bool,
    pub ccache: Option<String>,
    /// Call Build::ninja() instead of this.
    pub ninja_in_file: bool,
    pub verbose: usize,
    pub submodules: Option<bool>,
    pub compiler_docs: bool,
    pub library_docs_private_items: bool,
    pub docs_minification: bool,
    pub docs: bool,
    pub locked_deps: bool,
    pub vendor: bool,
    pub target_config: HashMap<TargetSelection, Target>,
    pub full_bootstrap: bool,
    pub bootstrap_cache_path: Option<PathBuf>,
    pub extended: bool,
    pub tools: Option<HashSet<String>>,
    pub sanitizers: bool,
    pub profiler: bool,
    pub omit_git_hash: bool,
    pub skip: Vec<PathBuf>,
    pub include_default_paths: bool,
    pub rustc_error_format: Option<String>,
    pub json_output: bool,
    pub test_compare_mode: bool,
    pub color: Color,
    pub patch_binaries_for_nix: Option<bool>,
    pub stage0_path: Option<PathBuf>,
    pub stage0_metadata: build_helper::stage0_parser::Stage0,
    pub android_ndk: Option<PathBuf>,
    /// Whether to use the `c` feature of the `compiler_builtins` crate.
    pub optimized_compiler_builtins: bool,

    pub stdout_is_tty: bool,
    pub stderr_is_tty: bool,

    pub on_fail: Option<String>,
    pub stage: u32,
    pub keep_stage: Vec<u32>,
    pub keep_stage_std: Vec<u32>,
    pub src: PathBuf,
    /// defaults to `config.toml`
    pub config: Option<PathBuf>,
    pub jobs: Option<u32>,
    pub cmd: Subcommand,
    pub incremental: bool,
    pub dry_run: DryRun,
    pub dump_bootstrap_shims: bool,
    /// Arguments appearing after `--` to be forwarded to tools,
    /// e.g. `--fix-broken` or test arguments.
    pub free_args: Vec<String>,

    /// `None` if we shouldn\'t download CI compiler artifacts, or the commit to download if we should.
    #[cfg(not(test))]
    download_rustc_commit: Option<String>,
    #[cfg(test)]
    pub download_rustc_commit: Option<String>,

    pub deny_warnings: bool,
    pub backtrace_on_ice: bool,

    // llvm codegen options
    pub llvm_assertions: bool,
    pub llvm_tests: bool,
    pub llvm_enzyme: bool,
    pub llvm_offload: bool,
    pub llvm_plugins: bool,
    pub llvm_optimize: bool,
    pub llvm_thin_lto: bool,
    pub llvm_release_debuginfo: bool,
    pub llvm_static_stdcpp: bool,
    pub llvm_libzstd: bool,
    /// `None` if `llvm_from_ci` is true and we haven\'t yet downloaded llvm.
    #[cfg(not(test))]
    llvm_link_shared: Cell<Option<bool>>,
    #[cfg(test)]
    pub llvm_link_shared: Cell<Option<bool>>,
    pub llvm_clang_cl: Option<String>,
    pub llvm_targets: Option<String>,
    pub llvm_experimental_targets: Option<String>,
    pub llvm_link_jobs: Option<u32>,
    pub llvm_version_suffix: Option<String>,
    pub llvm_use_linker: Option<String>,
    pub llvm_allow_old_toolchain: bool,
    pub llvm_polly: bool,
    pub llvm_clang: bool,
    pub llvm_enable_warnings: bool,
    pub llvm_from_ci: bool,
    pub llvm_build_config: HashMap<String, String>,
    pub llvm_enable_projects: Option<String>,

    pub lld_mode: LldMode,
    pub lld_enabled: bool,
    pub llvm_tools_enabled: bool,
    pub llvm_bitcode_linker_enabled: bool,

    pub llvm_cflags: Option<String>,
    pub llvm_cxxflags: Option<String>,
    pub llvm_ldflags: Option<String>,
    pub llvm_use_libcxx: bool,

    // rust codegen options
    pub rust_optimize: RustOptimize,
    pub rust_codegen_units: Option<u32>,
    pub rust_codegen_units_std: Option<u32>,

    pub rustc_debug_assertions: bool,
    pub std_debug_assertions: bool,

    pub rust_overflow_checks: bool,
    pub rust_overflow_checks_std: bool,
    pub rust_debug_logging: bool,
    pub rust_debuginfo_level_rustc: DebuginfoLevel,
    pub rust_debuginfo_level_std: DebuginfoLevel,
    pub rust_debuginfo_level_tools: DebuginfoLevel,
    pub rust_debuginfo_level_tests: DebuginfoLevel,
    pub rust_rpath: bool,
    pub rust_strip: bool,
    pub rust_frame_pointers: bool,
    pub rust_stack_protector: Option<String>,
    pub rustc_default_linker: Option<String>,
    pub rust_optimize_tests: bool,
    pub rust_dist_src: bool,
    pub rust_codegen_backends: Vec<String>,
    pub rust_verify_llvm_ir: bool,
    pub rust_thin_lto_import_instr_limit: Option<u32>,
    pub rust_randomize_layout: bool,
    pub rust_remap_debuginfo: bool,
    pub rust_new_symbol_mangling: Option<bool>,
    pub rust_profile_use: Option<String>,
    pub rust_profile_generate: Option<String>,
    pub rust_lto: RustcLto,
    pub rust_validate_mir_opts: Option<u32>,
    pub rust_std_features: BTreeSet<String>,
    pub llvm_profile_use: Option<String>,
    pub llvm_profile_generate: bool,
    pub llvm_libunwind_default: Option<LlvmLibunwind>,
    pub enable_bolt_settings: bool,

    pub reproducible_artifacts: Vec<String>,

    pub build: TargetSelection,
    pub hosts: Vec<TargetSelection>,
    pub targets: Vec<TargetSelection>,
    pub local_rebuild: bool,
    pub jemalloc: bool,
    pub control_flow_guard: bool,
    pub ehcont_guard: bool,

    // dist misc
    pub dist_sign_folder: Option<PathBuf>,
    pub dist_upload_addr: Option<String>,
    pub dist_compression_formats: Option<Vec<String>>,
    pub dist_compression_profile: String,
    pub dist_include_mingw_linker: bool,
    pub dist_vendor: bool,

    // libstd features
    pub backtrace: bool, // support for RUST_BACKTRACE

    // misc
    pub low_priority: bool,
    pub channel: String,
    pub description: Option<String>,
    pub verbose_tests: bool,
    pub save_toolstates: Option<PathBuf>,
    pub print_step_timings: bool,
    pub print_step_rusage: bool,

    // Fallback musl-root for all targets
    pub musl_root: Option<PathBuf>,
    pub prefix: Option<PathBuf>,
    pub sysconfdir: Option<PathBuf>,
    pub datadir: Option<PathBuf>,
    pub docdir: Option<PathBuf>,
    pub bindir: PathBuf,
    pub libdir: Option<PathBuf>,
    pub mandir: Option<PathBuf>,
    pub codegen_tests: bool,
    pub nodejs: Option<PathBuf>,
    pub npm: Option<PathBuf>,
    pub gdb: Option<PathBuf>,
    pub lldb: Option<PathBuf>,
    pub python: Option<PathBuf>,
    pub reuse: Option<PathBuf>,
    pub cargo_native_static: bool,
    pub configure_args: Vec<String>,
    pub out: PathBuf,
    pub rust_info: channel::GitInfo,

    pub cargo_info: channel::GitInfo,
    pub rust_analyzer_info: channel::GitInfo,
    pub clippy_info: channel::GitInfo,
    pub miri_info: channel::GitInfo,
    pub rustfmt_info: channel::GitInfo,
    pub enzyme_info: channel::GitInfo,
    pub in_tree_llvm_info: channel::GitInfo,
    pub in_tree_gcc_info: channel::GitInfo,

    // These are either the stage0 downloaded binaries or the locally installed ones.
    pub initial_cargo: PathBuf,
    pub initial_rustc: PathBuf,
    pub initial_cargo_clippy: Option<PathBuf>,

    #[cfg(not(test))]
    initial_rustfmt: RefCell<RustfmtState>,
    #[cfg(test)]
    pub initial_rustfmt: RefCell<RustfmtState>,

    pub ci: CiConfig,

    /// The paths to work with. For example: with `./x check foo bar` we get
    /// `paths=[\"foo\", \"bar\"]`.
    pub paths: Vec<PathBuf>,

    /// Command for visual diff display, e.g. `diff-tool --color=always`.
    pub compiletest_diff_tool: Option<String>,
}

impl From<ParsedConfig> for Config {
    fn from(parsed_config: ParsedConfig) -> Self {
        let mut config = Config::default();

        config.change_id = parsed_config.change_id.and_then(|s| s.parse().ok());
        config.rustc_source = parsed_config.rustc_source.map(PathBuf::from);
        config.bypass_bootstrap_lock = parsed_config.bypass_bootstrap_lock;
        config.verbose = parsed_config.verbose.unwrap_or(0);
        config.submodules = parsed_config.submodules;
        config.compiler_docs = parsed_config.compiler_docs.unwrap_or_default();
        config.library_docs_private_items =
            parsed_config.library_docs_private_items.unwrap_or_default();
        config.docs_minification = parsed_config.docs_minification.unwrap_or_default();
        config.docs = parsed_config.docs.unwrap_or_default();
        config.locked_deps = parsed_config.locked_deps.unwrap_or_default();
        config.vendor = parsed_config.vendor.unwrap_or_default();
        // config.target_config: HashMap<TargetSelection, Target>, // Complex, will need to be handled separately
        config.full_bootstrap = parsed_config.full_bootstrap.unwrap_or_default();
        config.bootstrap_cache_path = parsed_config.bootstrap_cache_path;
        config.extended = parsed_config.extended.unwrap_or_default();
        config.tools = parsed_config.tools;
        config.sanitizers = parsed_config.sanitizers.unwrap_or_default();
        config.profiler = parsed_config.profiler.unwrap_or_default();
        config.omit_git_hash = parsed_config.omit_git_hash;
        // config.skip: Vec<PathBuf>, // Needs mapping from Vec<String> if ParsedConfig has it
        config.include_default_paths = true; // Default value
        // config.rustc_error_format: Option<String>, // Direct map
        // config.json_output: bool, // Direct map
        // config.test_compare_mode: bool, // Direct map
        // config.color: Color, // Needs conversion
        config.patch_binaries_for_nix = parsed_config.patch_binaries_for_nix;
        // config.stage0_path: Option<PathBuf>, // Direct map
        // config.stage0_metadata: build_helper::stage0_parser::Stage0, // Complex
        config.android_ndk = parsed_config.android_ndk;
        config.optimized_compiler_builtins = parsed_config.optimized_compiler_builtins;

        config.stdout_is_tty = parsed_config.stdout_is_tty.unwrap_or_default();
        config.stderr_is_tty = parsed_config.stderr_is_tty.unwrap_or_default();

        // config.on_fail: Option<String>, // Direct map
        config.stage = parsed_config.stage.unwrap_or(0) as u32;
        // config.keep_stage: Vec<u32>, // Needs mapping
        // config.keep_stage_std: Vec<u32>, // Needs mapping
        config.src = parsed_config.src;
        config.config = parsed_config.config;
        config.jobs = parsed_config.jobs.map(|j| j as u32);
        // config.cmd: Subcommand, // Needs conversion
        config.incremental = parsed_config.incremental;
        config.dry_run = parsed_config.dry_run;
        config.dump_bootstrap_shims = false; // Default value
        // config.free_args: Vec<String>, // Direct map

        config.download_rustc_commit = parsed_config.download_rustc_commit;

        config.deny_warnings = parsed_config.deny_warnings.unwrap_or_default();
        config.backtrace_on_ice = parsed_config.backtrace_on_ice.unwrap_or_default();

        config.llvm_assertions = parsed_config.llvm_assertions.unwrap_or_default();
        config.llvm_tests = parsed_config.llvm_tests;
        config.llvm_enzyme = parsed_config.llvm_enzyme_flag.unwrap_or_default();
        config.llvm_offload = parsed_config.llvm_offload;
        config.llvm_plugins = parsed_config.llvm_plugins;
        config.llvm_optimize = parsed_config.llvm_optimize.unwrap_or_default();
        config.llvm_thin_lto = parsed_config.llvm_thin_lto;
        config.llvm_release_debuginfo = parsed_config.llvm_release_debuginfo.unwrap_or_default();
        config.llvm_static_stdcpp = parsed_config.llvm_static_stdcpp.unwrap_or_default();
        config.llvm_libzstd = parsed_config.llvm_libzstd.unwrap_or_default();
        // config.llvm_link_shared: Cell<Option<bool>>, // Complex
        config.llvm_clang_cl = parsed_config.llvm_clang_cl;
        // config.llvm_targets: Option<String>, // Needs conversion from Vec<String>
        // config.llvm_experimental_targets: Option<String>, // Needs conversion from Vec<String>
        config.llvm_link_jobs = parsed_config.llvm_link_jobs.map(|j| j as u32);
        config.llvm_version_suffix = parsed_config.llvm_version_suffix;
        config.llvm_use_linker = parsed_config.llvm_use_linker;
        config.llvm_allow_old_toolchain = parsed_config.llvm_allow_old_toolchain;
        config.llvm_polly = parsed_config.llvm_polly;
        config.llvm_clang = parsed_config.llvm_clang;
        config.llvm_enable_warnings = parsed_config.llvm_enable_warnings;
        config.llvm_from_ci = parsed_config.llvm_from_ci;
        // config.llvm_build_config: HashMap<String, String>, // Complex
        // config.llvm_enable_projects: Option<String>, // Needs conversion from Vec<String>

        // config.lld_mode: LldMode, // Needs conversion
        config.lld_enabled = parsed_config.lld_enabled;
        config.llvm_tools_enabled = parsed_config.llvm_tools_enabled;
        config.llvm_bitcode_linker_enabled =
            parsed_config.llvm_bitcode_linker_enabled.unwrap_or_default();

        // config.llvm_cflags: Option<String>, // Direct map
        // config.llvm_cxxflags: Option<String>, // Direct map
        // config.llvm_ldflags: Option<String>, // Direct map
        config.llvm_use_libcxx = parsed_config.llvm_use_libcxx.unwrap_or_default();

        // config.rust_optimize: RustOptimize, // Needs conversion
        config.rust_codegen_units = parsed_config.rust_codegen_units.map(|u| u as u32);
        config.rust_codegen_units_std = parsed_config.rust_codegen_units_std.map(|u| u as u32);

        config.rustc_debug_assertions = parsed_config.rustc_debug_assertions;
        config.std_debug_assertions = parsed_config.std_debug_assertions;

        config.rust_overflow_checks = parsed_config.rust_overflow_checks;
        config.rust_overflow_checks_std = parsed_config.rust_overflow_checks_std;
        config.rust_debug_logging = parsed_config.rust_debug_logging;
        // config.rust_debuginfo_level_rustc: DebuginfoLevel, // Needs conversion
        // config.rust_debuginfo_level_std: DebuginfoLevel, // Needs conversion
        // config.rust_debuginfo_level_tools: DebuginfoLevel, // Needs conversion
        // config.rust_debuginfo_level_tests: DebuginfoLevel, // Needs conversion
        config.rust_rpath = parsed_config.rust_rpath.unwrap_or_default();
        config.rust_strip = parsed_config.rust_strip.unwrap_or_default();
        config.rust_frame_pointers = parsed_config.rust_frame_pointers.unwrap_or_default();
        // config.rust_stack_protector: Option<String>, // Direct map
        config.rustc_default_linker = parsed_config.rustc_default_linker;
        config.rust_optimize_tests = parsed_config.rust_optimize_tests.unwrap_or_default();
        config.rust_dist_src = parsed_config.rust_dist_src.unwrap_or_default();
        config.rust_codegen_backends = parsed_config.rust_codegen_backends;
        config.rust_verify_llvm_ir = parsed_config.rust_verify_llvm_ir.unwrap_or_default();
        config.rust_thin_lto_import_instr_limit = parsed_config.rust_thin_lto_import_instr_limit;
        config.rust_randomize_layout = parsed_config.rust_randomize_layout;
        config.rust_remap_debuginfo = parsed_config.rust_remap_debuginfo.unwrap_or_default();
        config.rust_new_symbol_mangling = parsed_config.rust_new_symbol_mangling;
        // config.rust_profile_use: Option<String>, // Direct map
        config.rust_profile_generate = parsed_config.rust_profile_generate.map(|b| b.to_string());
        // config.rust_lto: RustcLto, // Needs conversion
        // config.rust_validate_mir_opts: Option<u32>, // Direct map
        config.rust_std_features = parsed_config.rust_std_features;
        // config.llvm_profile_use: Option<String>, // Direct map
        config.llvm_profile_generate = parsed_config.llvm_profile_generate;
        // config.llvm_libunwind_default: Option<LlvmLibunwind>, // Needs conversion
        config.enable_bolt_settings = false; // Default value

        // config.reproducible_artifacts: Vec<String>, // Direct map

        config.build = parsed_config.build;
        config.hosts = parsed_config.hosts.into_iter().map(TargetSelection::from).collect();
        config.targets = parsed_config.targets.into_iter().map(TargetSelection::from).collect();
        config.local_rebuild = parsed_config.local_rebuild.unwrap_or_default();
        config.jemalloc = parsed_config.jemalloc.unwrap_or_default();
        config.control_flow_guard = parsed_config.control_flow_guard.unwrap_or_default();
        config.ehcont_guard = parsed_config.ehcont_guard.unwrap_or_default();

        config.dist_sign_folder = parsed_config.dist_sign_folder;
        config.dist_upload_addr = parsed_config.dist_upload_addr;
        config.dist_compression_formats = parsed_config.dist_compression_formats;
        config.dist_compression_profile =
            parsed_config.dist_compression_profile.unwrap_or_default();
        config.dist_include_mingw_linker =
            parsed_config.dist_include_mingw_linker.unwrap_or_default();
        config.dist_vendor = parsed_config.dist_vendor;

        config.backtrace = parsed_config.backtrace.unwrap_or_default();

        config.low_priority = parsed_config.low_priority.unwrap_or_default();
        config.channel = parsed_config.channel.unwrap_or_default();
        config.description = parsed_config.description;
        config.verbose_tests = parsed_config.verbose_tests_flag.unwrap_or_default();
        config.save_toolstates = parsed_config.save_toolstates;
        config.print_step_timings = parsed_config.print_step_timings.unwrap_or_default();
        config.print_step_rusage = parsed_config.print_step_rusage.unwrap_or_default();

        config.musl_root = parsed_config.musl_root;
        config.prefix = parsed_config.prefix;
        config.sysconfdir = parsed_config.sysconfdir;
        config.datadir = parsed_config.datadir;
        config.docdir = parsed_config.docdir;
        config.bindir = parsed_config.bindir.unwrap_or_default();
        config.libdir = parsed_config.libdir;
        config.mandir = parsed_config.mandir;
        config.codegen_tests = parsed_config.codegen_tests.unwrap_or_default();
        config.nodejs = parsed_config.nodejs;
        config.npm = parsed_config.npm;
        config.gdb = parsed_config.gdb;
        config.lldb = parsed_config.lldb;
        config.python = parsed_config.python;
        config.reuse = parsed_config.reuse;
        config.cargo_native_static = parsed_config.cargo_native_static.unwrap_or_default();
        // config.configure_args: Vec<String>, // Direct map
        config.out = parsed_config.out_dir.unwrap_or_else(|| "build".into());
        // config.rust_info: channel::GitInfo, // Complex

        // config.cargo_info: channel::GitInfo, // Complex
        // config.rust_analyzer_info: channel::GitInfo, // Complex
        // config.clippy_info: channel::GitInfo, // Complex
        // config.miri_info: channel::GitInfo, // Complex
        // config.rustfmt_info: channel::GitInfo, // Complex
        // config.enzyme_info: channel::GitInfo, // Complex
        // config.in_tree_llvm_info: channel::GitInfo, // Complex
        // config.in_tree_gcc_info: channel::GitInfo, // Complex

        config.initial_cargo = parsed_config.initial_cargo.unwrap_or_default();
        config.initial_rustc = parsed_config.initial_rustc.unwrap_or_default();
        config.initial_cargo_clippy = parsed_config
            .initial_cargo_clippy
            .map(|b| if b { Some(PathBuf::from("true")) } else { None })
            .flatten();

        // config.initial_rustfmt: RefCell<RustfmtState>, // Complex

        // config.ci: CiConfig, // Complex

        config.paths = parsed_config.paths;

        config.compiletest_diff_tool = parsed_config.compiletest_diff_tool;

        // Nix-related fields
        config.nixpkgs_path = parsed_config.nixpkgs_path.map(PathBuf::from);
        config.rust_overlay_path = parsed_config.rust_overlay_path.map(PathBuf::from);
        config.rust_bootstrap_nix_path = parsed_config.rust_bootstrap_nix_path.map(PathBuf::from);
        config.configuration_nix_path = parsed_config.configuration_nix_path.map(PathBuf::from);
        config.rust_src_flake_path = parsed_config.rust_src_flake_path.map(PathBuf::from);
        config.rust_bootstrap_nix_flake_ref = parsed_config.rust_bootstrap_nix_flake_ref;
        config.rust_src_flake_ref = parsed_config.rust_src_flake_ref;

        config
    }
}
