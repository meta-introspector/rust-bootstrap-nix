use crate::prelude::*;
use std::process::Command;
use std::env;
use anyhow::{Context, Result};

/// Global configuration for the entire build and/or bootstrap.
///
/// This structure is parsed from `config.toml`, and some of the fields are inferred from `git` or build-time parameters.
///
/// Note that this structure is not decoded directly into, but rather it is
/// filled out from the decoded forms of the structs below. For documentation
/// each field, see the corresponding fields in
/// `config.example.toml`.
#[derive(Default, Clone)]
pub struct Config {
    pub change_id: Option<usize>,
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
    pub nixpkgs_path: Option<PathBuf>,
    pub rust_overlay_path: Option<PathBuf>,
    pub rust_bootstrap_nix_path: Option<PathBuf>,
    pub configuration_nix_path: Option<PathBuf>,
    pub rust_src_flake_path: Option<PathBuf>,
    pub jobs: Option<u32>,
    pub cmd: Subcommand,
    pub incremental: bool,
    pub dry_run: DryRun,
    pub dump_bootstrap_shims: bool,
    /// Arguments appearing after `--` to be forwarded to tools,
    /// e.g. `--fix-broken` or test arguments.
    pub free_args: Vec<String>,

    /// `None` if we shouldn't download CI compiler artifacts, or the commit to download if we should.
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
    /// `None` if `llvm_from_ci` is true and we haven't yet downloaded llvm.
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
    /// `paths=["foo", "bar"]`.
    pub paths: Vec<PathBuf>,

    /// Command for visual diff display, e.g. `diff-tool --color=always`.
    pub compiletest_diff_tool: Option<String>,

    pub fn resolve_nix_paths(&mut self) -> Result<()> {
        // Helper to get flake path
        let get_flake_path = |flake_url: &str| -> Result<PathBuf> {
            let output = Command::new("nix")
                .arg("flake")
                .arg("prefetch")
                .arg(flake_url)
                .arg("--json")
                .output()
                .context(format!("Failed to execute 'nix flake prefetch {}'", flake_url))?;

            if !output.status.success() {
                anyhow::bail!(
                    "nix flake prefetch failed for {}: {}",
                    flake_url,
                    String::from_utf8_lossy(&output.stderr)
                );
            }

            let json_output: serde_json::Value = serde_json::from_slice(&output.stdout)
                .context(format!("Failed to parse JSON output from nix flake prefetch for {}", flake_url))?;

            let path = json_output["path"]
                .as_str()
                .context(format!("'path' field not found in nix flake prefetch output for {}", flake_url))?
                .into();
            Ok(path)
        };

        // Resolve nixpkgs_path
        if self.nixpkgs_path.is_none() {
            let nixpkgs_rev = "26833ad1dad83826ef7cc52e0009ca9b7097c79f"; // From configuration-nix/flake.lock
            let nixpkgs_url = format!("github:meta-introspector/nixpkgs?rev={}", nixpkgs_rev);
            self.nixpkgs_path = Some(get_flake_path(&nixpkgs_url)?);
        }

        // Resolve rust_overlay_path
        if self.rust_overlay_path.is_none() {
            let rust_overlay_rev = "eee7767f08f58eb56822d7e85423098eb3e6dd65"; // From configuration-nix/flake.lock
            let rust_overlay_url = format!("github:meta-introspector/rust-overlay?rev={}", rust_overlay_rev);
            self.rust_overlay_path = Some(get_flake_path(&rust_overlay_url)?);
        }

        // Resolve rust_src_flake_path
        if self.rust_src_flake_path.is_none() {
            let rust_src_flake_rev = "3487cd3843083db70ee30023f19344568ade9c9f"; // From configuration-nix/flake.lock
            let rust_src_flake_url = format!("github:meta-introspector/rust?rev={}", rust_src_flake_rev);
            self.rust_src_flake_path = Some(get_flake_path(&rust_src_flake_url)?);
        }

        // For local paths, assume current directory or relative path
        if self.rust_bootstrap_nix_path.is_none() {
            self.rust_bootstrap_nix_path = Some(env::current_dir()?);
        }
        if self.configuration_nix_path.is_none() {
            self.configuration_nix_path = Some(env::current_dir()?.join("configuration-nix"));
        }

        Ok(())
    }

}
