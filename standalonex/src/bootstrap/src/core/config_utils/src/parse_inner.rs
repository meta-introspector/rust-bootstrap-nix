use crate::ParsedConfig;
use std::path::{Path, PathBuf};
use std::env;

use crate::parse_inner_flags;
use crate::parse_inner_stage0;
use crate::parse_inner_toml;
use crate::parse_inner_src;
use crate::parse_inner_out;
use crate::ConfigApplicator;
use crate::ci_config;
use crate::build_config;
use crate::install_config;
use crate::llvm_assertions_config::LlvmAssertionsConfigApplicator;
use crate::rust_channel_git_hash_config::RustChannelGitHashConfigApplicator;

use crate::{LocalFlags, LocalTomlConfig};
use bootstrap::TargetSelection;

pub(crate) fn parse_inner(
    mut flags: LocalFlags,
    get_toml: impl Fn(&Path) -> Result<LocalTomlConfig, toml::de::Error>,
) -> ParsedConfig {
    let mut config = ParsedConfig::default();

    // Set flags.
    parse_inner_flags::parse_inner_flags(&mut config, &mut flags);

    // Infer the rest of the configuration.
    let build_src_from_toml = None; // This needs to be handled differently if it's coming from toml.build.src
    parse_inner_src::parse_inner_src(&mut config, &flags, &build_src_from_toml);

    parse_inner_out::parse_inner_out(&mut config);

    let mut toml = parse_inner_toml::parse_inner_toml(&mut config, &flags, get_toml);

    // Apply various configuration applicators
    let mut applicators: Vec<Box<dyn ConfigApplicator>> = Vec::new();
    applicators.push(Box::new(ci_config::CiConfigApplicator));
    applicators.push(Box::new(build_config::BuildConfigApplicator));
    applicators.push(Box::new(install_config::InstallConfigApplicator));
    applicators.push(Box::new(crate::llvm_assertions_config::LlvmAssertionsConfigApplicator));
    applicators.push(Box::new(crate::rust_channel_git_hash_config::RustChannelGitHashConfigApplicator));

    for applicator in applicators.iter() {
        applicator.apply_to_config(&mut config, &toml);
    }

    // Handle rust-specific configurations
    if let Some(rust_config) = toml.rust {
        config.rust_optimize = rust_config.optimize;
        config.rustc_debug_assertions = rust_config.rustc_debug_assertions.unwrap_or(false);
        config.std_debug_assertions = rust_config.std_debug_assertions.unwrap_or(config.rustc_debug_assertions);
        config.rust_overflow_checks = rust_config.overflow_checks.unwrap_or(false);
        config.rust_overflow_checks_std = rust_config.overflow_checks_std.unwrap_or(config.rust_overflow_checks);
        config.rust_debug_logging = rust_config.debug_logging.unwrap_or(config.rustc_debug_assertions);
        config.rust_debuginfo_level_rustc = rust_config.debuginfo_level_rustc.or(rust_config.debuginfo_level);
        config.rust_debuginfo_level_std = rust_config.debuginfo_level_std.or(rust_config.debuginfo_level);
        config.rust_debuginfo_level_tools = rust_config.debuginfo_level_tools.or(rust_config.debuginfo_level);
        config.rust_debuginfo_level_tests = rust_config.debuginfo_level_tests.unwrap_or_default();
        config.lld_enabled = rust_config.lld.unwrap_or(false);
        config.rust_std_features = rust_config.std_features.unwrap_or_default();

        config.rust_new_symbol_mangling = rust_config.new_symbol_mangling;
        config.rust_optimize_tests = rust_config.optimize_tests;
        config.rust_rpath = rust_config.rpath;
        config.rust_strip = rust_config.strip;
        config.rust_frame_pointers = rust_config.frame_pointers;
        config.rust_stack_protector = rust_config.stack_protector;
        config.jemalloc = rust_config.jemalloc;
        config.test_compare_mode = rust_config.test_compare_mode;
        config.backtrace = rust_config.backtrace;
        config.description = rust_config.description;
        config.rust_dist_src = rust_config.dist_src;
        config.verbose_tests_flag = rust_config.verbose_tests;
        if let Some(true) = rust_config.incremental {
            config.incremental = true;
        }
        config.lld_mode = rust_config.lld_mode;
        config.llvm_bitcode_linker_enabled = rust_config.llvm_bitcode_linker;

        config.rust_randomize_layout = rust_config.randomize_layout.unwrap_or_default();
        config.llvm_tools_enabled = rust_config.llvm_tools.unwrap_or(true);

        if rust_config.parallel_compiler.is_some() {
            // WARNING: The `rust.parallel-compiler` option is deprecated and does nothing. The parallel compiler (with one thread) is now the default
        }

        config.llvm_enzyme_flag = rust_config.enzyme;
        config.rustc_default_linker = rust_config.default_linker;
        config.musl_root = rust_config.musl_root.map(PathBuf::from);
        config.save_toolstates = rust_config.save_toolstates.map(PathBuf::from);
        config.deny_warnings = rust_config.deny_warnings;
        config.backtrace_on_ice = rust_config.backtrace_on_ice;
        config.rust_verify_llvm_ir = rust_config.verify_llvm_ir;
        config.rust_thin_lto_import_instr_limit = rust_config.thin_lto_import_instr_limit;
        config.rust_remap_debuginfo = rust_config.remap_debuginfo;
        config.control_flow_guard = rust_config.control_flow_guard;
        config.ehcont_guard = rust_config.ehcont_guard;
        config.llvm_libunwind_default = rust_config.llvm_libunwind;

        if let Some(backends) = rust_config.codegen_backends {
            config.rust_codegen_backends = backends;
        }

        config.rust_codegen_units = rust_config.codegen_units;
        config.rust_codegen_units_std = rust_config.codegen_units_std;
        config.rust_profile_use = flags.rust_profile_use.or(rust_config.profile_use);
        config.rust_profile_generate = flags.rust_profile_generate.or(rust_config.profile_generate);
        config.rust_lto = rust_config.lto;
        config.rust_validate_mir_opts = rust_config.validate_mir_opts;
        config.download_rustc_commit = rust_config.download_rustc.map(|_| "some_commit".to_string()); // Placeholder
    } else {
        config.rust_profile_use = flags.rust_profile_use;
        config.rust_profile_generate = flags.rust_profile_generate;
    }

    // Handle llvm-specific configurations
    if let Some(llvm_config) = toml.llvm {
        config.llvm_optimize = llvm_config.optimize.unwrap_or(true);
        config.llvm_thin_lto = llvm_config.thin_lto;
        config.llvm_release_debuginfo = llvm_config.release_debuginfo;
        config.llvm_tests = llvm_config.tests.unwrap_or(false);
        config.llvm_enzyme_flag = llvm_config.enzyme;
        config.llvm_offload = llvm_config.offload;
        config.llvm_plugins = llvm_config.plugins;
        config.ccache = llvm_config.ccache;
        config.llvm_static_stdcpp = llvm_config.static_libstdcpp;
        config.llvm_libzstd = llvm_config.libzstd;
        config.ninja_in_file = llvm_config.ninja.unwrap_or(true);
        config.llvm_targets = llvm_config.targets;
        config.llvm_experimental_targets = llvm_config.experimental_targets;
        config.llvm_link_jobs = llvm_config.link_jobs;
        config.llvm_version_suffix = llvm_config.version_suffix;
        config.llvm_clang_cl = llvm_config.clang_cl;
        config.llvm_enable_projects = llvm_config.enable_projects;
        config.llvm_cflags = llvm_config.cflags;
        config.llvm_cxxflags = llvm_config.cxxflags;
        config.llvm_ldflags = llvm_config.ldflags;
        config.llvm_use_libcxx = llvm_config.use_libcxx;
        config.llvm_use_linker = llvm_config.use_linker;
        config.llvm_allow_old_toolchain = llvm_config.allow_old_toolchain.unwrap_or(false);
        config.llvm_polly = llvm_config.polly.unwrap_or(false);
        config.llvm_clang = llvm_config.clang.unwrap_or(false);
        config.llvm_enable_warnings = llvm_config.enable_warnings.unwrap_or(false);
        config.llvm_build_config = llvm_config.build_config.unwrap_or_default();
        config.llvm_from_ci = llvm_config.download_ci_llvm;
    }

    // Handle dist-specific configurations
    if let Some(dist_config) = toml.dist {
        config.dist_sign_folder = dist_config.sign_folder.map(PathBuf::from);
        config.dist_upload_addr = dist_config.upload_addr;
        config.dist_compression_formats = dist_config.compression_formats;
        config.dist_compression_profile = dist_config.compression_profile;
        config.rust_dist_src = dist_config.src_tarball;
        config.dist_include_mingw_linker = dist_config.include_mingw_linker;
        config.dist_vendor = dist_config.vendor;
    }

    // Handle target-specific configurations
    if let Some(target_configs) = toml.target {
        for (triple, cfg) in target_configs {
            let mut target = crate::LocalTargetConfig::default(); // Assuming LocalTargetConfig is defined
            target.llvm_config = cfg.llvm_config.map(PathBuf::from);
            target.llvm_has_rust_patches = cfg.llvm_has_rust_patches;
            target.llvm_filecheck = cfg.llvm_filecheck.map(PathBuf::from);
            target.llvm_libunwind = cfg.llvm_libunwind;
            target.no_std = cfg.no_std;
            target.cc = cfg.cc.map(PathBuf::from);
            target.cxx = cfg.cxx.map(PathBuf::from);
            target.ar = cfg.ar.map(PathBuf::from);
            target.ranlib = cfg.ranlib.map(PathBuf::from);
            target.linker = cfg.linker.map(PathBuf::from);
            target.crt_static = cfg.crt_static;
            target.musl_root = cfg.musl_root.map(PathBuf::from);
            target.musl_libdir = cfg.musl_libdir.map(PathBuf::from);
            target.wasi_root = cfg.wasi_root.map(PathBuf::from);
            target.qemu_rootfs = cfg.qemu_rootfs.map(PathBuf::from);
            target.runner = cfg.runner;
            target.sanitizers = cfg.sanitizers;
            target.profiler = cfg.profiler;
            target.rpath = cfg.rpath;
            target.codegen_backends = cfg.codegen_backends;
            target.split_debuginfo = cfg.split_debuginfo;
            config.target_config.insert(TargetSelection::from_user(&triple), target);
        }
    }

    config
}

fn apply_test_config(config: &mut ParsedConfig, toml: &mut LocalTomlConfig) {
    if cfg!(test) {
        let build = toml.build.get_or_insert_with(Default::default);
        build.rustc = build.rustc.take().or(std::env::var_os("RUSTC").map(|p| p.into()));
        build.cargo = build.cargo.take().or(std::env::var_os("CARGO").map(|p| p.into()));
    }
}