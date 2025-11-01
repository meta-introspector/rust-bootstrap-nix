#![macro_use]

use bootstrap_macros::t;
use build_helper::GitInfo;
use build_helper::RUSTC_IF_UNCHANGED_ALLOWED_PATHS;
use build_helper::channel;
use build_helper::ci::CiEnv;
use build_helper::exit;
use build_helper::git::GitConfig;
use build_helper::git::get_closest_merge_commit;
use build_helper::git::output_result;
//use build_helper::git::GitCommand;
use build_helper::git::Command;
use build_helper::helpers;
use build_helper::llvm;
use build_helper::output;
use build_helper::prelude::*;
use build_helper::prelude::*;
use config_core::ReplaceOpt;
use crate::CODEGEN_BACKEND_PREFIX;
use crate::ChangeIdWrapper;
use crate::Config;
use crate::DebuginfoLevel;
use crate::DocTests;
use crate::DryRun;
use crate::Flags;
use crate::Kind;
use crate::LldMode;
use crate::LlvmLibunwind;
use crate::OptimizeVisitor;
use crate::RustOptimize;
use crate::RustcLto;
use crate::RustfmtState;
use crate::SplitDebuginfo;
use crate::StringOrBool;
use crate::StringOrInt;
use crate::Target;
use crate::TargetSelection;
use crate::TargetSelectionList;
use crate::TomlConfig;
//use crate::get_toml;
use crate::check_incompatible_options_for_ci_rustc;
use crate::ci::Ci;
use crate::ciconfig::CiConfig;
use crate::dist::Dist;
use crate::env;
use crate::exe;
use crate::flags::Color;
use crate::flags::Warnings;
use crate::fs;
use crate::install::Install;
use crate::llvm::Llvm;
use crate::rust::Rust;
use crate::subcommand::Subcommand;
use crate::subcommand_groups::BuildTool::Build;
use crate::target_selection::target_selection_list;
use crate::tomltarget::TomlTarget;
use std::cell::*;
use std::cmp;
use std::collections::*;
use std::path::absolute;
use std::thread::Builder;
//use crate::macro_rules::check_ci_llvm;
//use crate::Kind::Build;
//use crate::subcommand::BuildTool::Build;
//use crate::subcommand::Subcommand::Build;
//use crate::subcommand_groups::BuildTool::Build;


macro_rules! check_ci_llvm {
    ($name:expr) => {
        assert!(
            $name.is_none(),
            "setting {} is incompatible with download-ci-llvm.",
            stringify!($name).replace("_", "-")
        );
    };
}



impl Config {
    pub(crate) fn parse_inner(
        mut flags: Flags,
        get_toml: impl Fn(&Path) -> Result<TomlConfig, toml::de::Error>,
    ) -> Config {
        let mut config = Config::default_opts();
        config.paths = std::mem::take(&mut flags.paths);
        config.skip = flags.skip.into_iter().chain(flags.exclude).collect();
        config.include_default_paths = flags.include_default_paths;
        config.rustc_error_format = flags.rustc_error_format;
        config.json_output = flags.json_output;
        config.on_fail = flags.on_fail;
        config.cmd = flags.cmd;
        config.incremental = flags.incremental;
        config.dry_run = if flags.dry_run {
            DryRun::UserSelected
        } else {
            DryRun::Disabled
        };
        config.dump_bootstrap_shims = flags.dump_bootstrap_shims;
        config.keep_stage = flags.keep_stage;
        config.keep_stage_std = flags.keep_stage_std;
        config.color = flags.color;
        config.free_args = std::mem::take(&mut flags.free_args);
        config.llvm_profile_use = flags.llvm_profile_use;
        config.llvm_profile_generate = flags.llvm_profile_generate;
        config.enable_bolt_settings = flags.enable_bolt_settings;
        config.bypass_bootstrap_lock = flags.bypass_bootstrap_lock;
        config.src = if let Some(src) = flags.src {
            src
        } else if let Some(src) = build_src_from_toml_field {
            src
        } else {
            let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            manifest_dir.parent().unwrap().parent().unwrap().to_owned()
        };
        if cfg!(test) {
            config.out = Path::new(
                    &env::var_os("CARGO_TARGET_DIR")
                        .expect("cargo test directly is not supported"),
                )
                .parent()
                .unwrap()
                .to_path_buf();
        }
        config.stage0_metadata = build_helper::stage0_parser::parse_stage0_file(
            &toml.stage0_path.expect("stage0_path must be set"),
        );
        let toml_path = flags
            .config
            .clone()
            .or_else(|| {
                env::var_os("RUST_BOOTSTRAP_GENERATED_CONFIG").map(PathBuf::from)
            })
            .or_else(|| env::var_os("RUST_BOOTSTRAP_CONFIG").map(PathBuf::from));
        let using_default_path = toml_path.is_none();
        let mut toml_path = toml_path.unwrap_or_else(|| PathBuf::from("config.toml"));
        if using_default_path && !toml_path.exists() {
            toml_path = config.src.join(toml_path);
        }
        let file_content = t!(fs::read_to_string(& config.ci.channel_file));
        let ci_channel = file_content.trim_end();
        let mut toml = if !using_default_path || toml_path.exists() {
            config.config = Some(
                if cfg!(not(feature = "bootstrap-self-test")) {
                    toml_path.canonicalize().unwrap()
                } else {
                    toml_path.clone()
                },
            );
            get_toml::get_toml(&toml_path)
                .unwrap_or_else(|e| {
                    eprintln!("ERROR: Failed to parse '{}': {e}", toml_path.display());
                    exit!(2);
                })
        } else {
            config.config = None;
            TomlConfig::default()
        };
        if cfg!(test) {
            let build = toml.build.get_or_insert_with(Default::default);
            build.rustc = build
                .rustc
                .take()
                .or(std::env::var_os("RUSTC").map(|p| p.into()));
            build.cargo = build
                .cargo
                .take()
                .or(std::env::var_os("CARGO").map(|p| p.into()));
        }
        if let Some(include) = &toml.profile {
            let profile_aliases = HashMap::from([("user", "dist")]);
            let include = match profile_aliases.get(include.as_str()) {
                Some(alias) => alias,
                None => include.as_str(),
            };
            let mut include_path = config.src.clone();
            include_path.push("src");
            include_path.push("bootstrap");
            include_path.push("defaults");
            include_path.push(format!("config.{include}.toml"));
            let included_toml = get_toml(&include_path)
                .unwrap_or_else(|e| {
                    eprintln!(
                        "ERROR: Failed to parse default config profile at '{}': {e}",
                        include_path.display()
                    );
                    exit!(2);
                });
            toml.merge(included_toml, ReplaceOpt::IgnoreDuplicate);
        }
        let mut override_toml = TomlConfig::default();
        for option in flags.set.iter() {
            pub fn get_table(option: &str) -> Result<TomlConfig, toml::de::Error> {
                toml::from_str(option)
                    .and_then(|table: toml::Value| TomlConfig::deserialize(table))
            }
            let mut err = match get_table(option) {
                Ok(v) => {
                    override_toml.merge(v, ReplaceOpt::ErrorOnDuplicate);
                    continue;
                }
                Err(e) => e,
            };
            if let Some((key, value)) = option.split_once('=') {
                if !value.contains('"') {
                    match get_table(&format!(r#"{key}="{value}""#)) {
                        Ok(v) => {
                            override_toml.merge(v, ReplaceOpt::ErrorOnDuplicate);
                            continue;
                        }
                        Err(e) => err = e,
                    }
                }
            }
            eprintln!("failed to parse override `{option}`: `{err}");
            exit!(2)
        }
        toml.merge(override_toml, ReplaceOpt::Override);
        let build_src_from_toml = toml.build.as_ref().and_then(|b| b.src.clone());
        let Ci { channel_file, version_file, tools_dir, llvm_project_dir, gcc_dir } = toml
            .ci
            .unwrap_or_default();
        set(&mut config.ci.channel_file, channel_file.map(PathBuf::from));
        set(&mut config.ci.version_file, version_file.map(PathBuf::from));
        set(&mut config.ci.tools_dir, tools_dir.map(PathBuf::from));
        set(&mut config.ci.llvm_project_dir, llvm_project_dir.map(PathBuf::from));
        set(&mut config.ci.gcc_dir, gcc_dir.map(PathBuf::from));
        config.change_id = toml.change_id.inner;
        if let Some(nix) = toml.nix {
            config.nixpkgs_path = nix.nixpkgs_path;
            config.rust_overlay_path = nix.rust_overlay_path;
            config.rust_bootstrap_nix_path = nix.rust_bootstrap_nix_path;
            config.configuration_nix_path = nix.configuration_nix_path;
            config.rust_src_flake_path = nix.rust_src_flake_path;
        }
        config.resolve_nix_paths().expect("Failed to resolve Nix paths");

        let Build {
            build,
            host,
            target,
            build_dir,
            cargo,
            rustc,
            rustfmt,
            cargo_clippy,
            docs,
            compiler_docs,
            library_docs_private_items,
            docs_minification,
            submodules,
            gdb,
            lldb,
            nodejs,
            npm,
            python,
            reuse,
            locked_deps,
            vendor,
            full_bootstrap,
            bootstrap_cache_path,
            extended,
            tools,
            verbose,
            sanitizers,
            profiler,
            cargo_native_static,
            low_priority,
            configure_args,
            local_rebuild,
            print_step_timings,
            print_step_rusage,
            check_stage,
            doc_stage,
            build_stage,
            test_stage,
            install_stage,
            dist_stage,
            bench_stage,
            patch_binaries_for_nix,
            metrics: _metrics,
            android_ndk,
            optimized_compiler_builtins,
            jobs,
            compiletest_diff_tool,
            src: build_src_from_toml_field,
        } = toml.build.unwrap_or_default();

        config.jobs = Some(threads_from_config(flags.jobs.unwrap_or(toml.build.as_ref().and_then(|b| b.jobs).unwrap_or(0))));
        if let Some(file_build) = toml.build {
            config.build = file_build;
        }
        set(&mut config.out, flags.build_dir.or_else(|| toml.build.as_ref().and_then(|b| b.build_dir).map(PathBuf::from)));
        if !config.out.is_absolute() {
            config.out = absolute(&config.out).expect("can't make empty path absolute");
        }
        if cargo_clippy.is_some() && rustc.is_none() {
            println!(
                "WARNING: Using `build.cargo-clippy` without `build.rustc` usually fails due to toolchain conflict."
            );
        }
        config.initial_cargo_clippy = toml.build.as_ref().and_then(|b| b.cargo_clippy);
        config.initial_rustc = if let Some(rustc) = toml.build.as_ref().and_then(|b| b.rustc) {
            if !flags.skip_stage0_validation {
                config.check_stage0_version(&rustc, "rustc");
            }
            rustc
        } else {
            config.download_beta_toolchain();
            config
                .out
                .join(config.build)
                .join("stage0")
                .join("bin")
                .join(exe("rustc", config.build))
        };
        config.initial_cargo = if let Some(cargo) = toml.build.as_ref().and_then(|b| b.cargo) {
            if !flags.skip_stage0_validation {
                config.check_stage0_version(&cargo, "cargo");
            }
            cargo
        } else {
            config.download_beta_toolchain();
            config
                .out
                .join(config.build)
                .join("stage0")
                .join("bin")
                .join(exe("cargo", config.build))
        };
        if config.dry_run {
            let dir = config.out.join("tmp-dry-run");
            t!(fs::create_dir_all(& dir));
            config.out = dir;
        }
        config.hosts = if let Some(TargetSelectionList(arg_host)) = flags.host {
            arg_host
        } else if let Some(file_host) = toml.build.as_ref().and_then(|b| b.host) {
            file_host.iter().map(|h| TargetSelection::from_user(h)).collect()
        } else {
            vec![config.build]
        };
        config.targets = if let Some(TargetSelectionList(arg_target)) = flags.target {
            arg_target
        } else if let Some(file_target) = toml.build.as_ref().and_then(|b| b.target) {
            file_target.iter().map(|h| TargetSelection::from_user(h)).collect()
        } else {
            config.hosts.clone()
        };
        config.nodejs = toml.build.as_ref().and_then(|b| b.nodejs).map(PathBuf::from);
        config.npm = toml.build.as_ref().and_then(|b| b.npm).map(PathBuf::from);
        config.gdb = toml.build.as_ref().and_then(|b| b.gdb).map(PathBuf::from);
        config.lldb = toml.build.as_ref().and_then(|b| b.lldb).map(PathBuf::from);
        config.python = toml.build.as_ref().and_then(|b| b.python).map(PathBuf::from);
        config.reuse = toml.build.as_ref().and_then(|b| b.reuse).map(PathBuf::from);
        config.submodules = toml.build.as_ref().and_then(|b| b.submodules);
        config.android_ndk = toml.build.as_ref().and_then(|b| b.android_ndk);
        config.bootstrap_cache_path = toml.build.as_ref().and_then(|b| b.bootstrap_cache_path);
        set(&mut config.low_priority, toml.build.as_ref().and_then(|b| b.low_priority));
        set(&mut config.compiler_docs, toml.build.as_ref().and_then(|b| b.compiler_docs));
        set(&mut config.library_docs_private_items, toml.build.as_ref().and_then(|b| b.library_docs_private_items));
        set(&mut config.docs_minification, toml.build.as_ref().and_then(|b| b.docs_minification));
        set(&mut config.docs, toml.build.as_ref().and_then(|b| b.docs));
        set(&mut config.locked_deps, toml.build.as_ref().and_then(|b| b.locked_deps));
        set(&mut config.vendor, toml.build.as_ref().and_then(|b| b.vendor));
        set(&mut config.full_bootstrap, toml.build.as_ref().and_then(|b| b.full_bootstrap));
        set(&mut config.extended, toml.build.as_ref().and_then(|b| b.extended));
        config.tools = toml.build.as_ref().and_then(|b| b.tools);
        set(&mut config.sanitizers, toml.build.as_ref().and_then(|b| b.sanitizers));
        set(&mut config.profiler, toml.build.as_ref().and_then(|b| b.profiler));
        set(&mut config.cargo_native_static, toml.build.as_ref().and_then(|b| b.cargo_native_static));
        set(&mut config.configure_args, toml.build.as_ref().and_then(|b| b.configure_args));
        set(&mut config.local_rebuild, toml.build.as_ref().and_then(|b| b.local_rebuild));
        set(&mut config.print_step_timings, toml.build.as_ref().and_then(|b| b.print_step_timings));
        set(&mut config.print_step_rusage, toml.build.as_ref().and_then(|b| b.print_step_rusage));
        config.patch_binaries_for_nix = toml.build.as_ref().and_then(|b| b.patch_binaries_for_nix);
        config.verbose = cmp::max(config.verbose, flags.verbose as usize);
        config.verbose_tests = config.is_verbose();
        if let Some(install) = toml.install {
            let Install {
                prefix,
                sysconfdir,
                docdir,
                bindir,
                libdir,
                mandir,
                datadir,
            } = install;
            config.prefix = prefix.map(PathBuf::from);
            config.sysconfdir = sysconfdir.map(PathBuf::from);
            config.datadir = datadir.map(PathBuf::from);
            config.docdir = docdir.map(PathBuf::from);
            if let Some(b) = bindir {
                config.bindir = PathBuf::from(b);
            } else if let Some(p) = &config.prefix {
                config.bindir = p.join("bin");
            }
            config.libdir = libdir.map(PathBuf::from);
            config.mandir = mandir.map(PathBuf::from);
        }
        config.llvm_assertions = toml
            .llvm
            .as_ref()
            .map_or(false, |llvm| llvm.assertions.unwrap_or(false));
        let mut llvm_tests = None;
        let mut llvm_enzyme = None;
        let mut llvm_offload = None;
        let mut llvm_plugins = None;
        let mut debug = None;
        let mut rustc_debug_assertions = None;
        let mut std_debug_assertions = None;
        let mut overflow_checks = None;
        let mut overflow_checks_std = None;
        let mut debug_logging = None;
        let mut debuginfo_level = None;
        let mut debuginfo_level_rustc = None;
        let mut debuginfo_level_std = None;
        let mut debuginfo_level_tools = None;
        let mut debuginfo_level_tests = None;
        let mut optimize = None;
        let mut lld_enabled = None;
        let mut std_features = None;
        let is_user_configured_rust_channel = if let Some(channel) = toml
            .rust
            .as_ref()
            .and_then(|r| r.channel.clone())
        {
            config.channel = channel;
            true
        } else {
            false
        };
        let default = config.channel == "dev";
        config.omit_git_hash = toml
            .rust
            .as_ref()
            .and_then(|r| r.omit_git_hash)
            .unwrap_or(default);
        config.rust_info = GitInfo::new(config.omit_git_hash, &config.src);
        config.cargo_info = GitInfo::new(
            config.omit_git_hash,
            &config.ci.tools_dir.join("cargo"),
        );
        config.rust_analyzer_info = GitInfo::new(
            config.omit_git_hash,
            &config.ci.tools_dir.join("rust-analyzer"),
        );
        config.clippy_info = GitInfo::new(
            config.omit_git_hash,
            &config.ci.tools_dir.join("clippy"),
        );
        config.miri_info = GitInfo::new(
            config.omit_git_hash,
            &config.ci.tools_dir.join("miri"),
        );
        config.rustfmt_info = GitInfo::new(
            config.omit_git_hash,
            &config.ci.tools_dir.join("rustfmt"),
        );
        config.enzyme_info = GitInfo::new(
            config.omit_git_hash,
            &config.ci.tools_dir.join("enzyme"),
        );
        config.in_tree_llvm_info = GitInfo::new(false, &config.ci.llvm_project_dir);
        config.in_tree_gcc_info = GitInfo::new(false, &config.ci.gcc_dir);
        if let Some(rust) = toml.rust {
            let Rust {
                optimize: optimize_toml,
                debug: debug_toml,
                codegen_units,
                codegen_units_std,
                rustc_debug_assertions: rustc_debug_assertions_toml,
                std_debug_assertions: std_debug_assertions_toml,
                overflow_checks: overflow_checks_toml,
                overflow_checks_std: overflow_checks_std_toml,
                debug_logging: debug_logging_toml,
                debuginfo_level: debuginfo_level_toml,
                debuginfo_level_rustc: debuginfo_level_rustc_toml,
                debuginfo_level_std: debuginfo_level_std_toml,
                debuginfo_level_tools: debuginfo_level_tools_toml,
                debuginfo_level_tests: debuginfo_level_tests_toml,
                backtrace,
                incremental,
                parallel_compiler,
                randomize_layout,
                default_linker,
                channel: _,
                description,
                musl_root,
                rpath,
                verbose_tests,
                optimize_tests,
                codegen_tests,
                omit_git_hash: _,
                dist_src,
                save_toolstates,
                codegen_backends,
                lld: lld_enabled_toml,
                llvm_tools,
                llvm_bitcode_linker,
                deny_warnings,
                backtrace_on_ice,
                verify_llvm_ir,
                thin_lto_import_instr_limit,
                remap_debuginfo,
                jemalloc,
                test_compare_mode,
                llvm_libunwind,
                control_flow_guard,
                ehcont_guard,
                new_symbol_mangling,
                profile_generate,
                profile_use,
                download_rustc,
                lto,
                validate_mir_opts,
                frame_pointers,
                stack_protector,
                strip,
                lld_mode,
                std_features: std_features_toml,
            } = rust;
            config.download_rustc_commit = config
                .download_ci_rustc_commit(download_rustc, config.llvm_assertions);
            debug = debug_toml;
            rustc_debug_assertions = rustc_debug_assertions_toml;
            std_debug_assertions = std_debug_assertions_toml;
            overflow_checks = overflow_checks_toml;
            overflow_checks_std = overflow_checks_std_toml;
            debug_logging = debug_logging_toml;
            debuginfo_level = debuginfo_level_toml;
            debuginfo_level_rustc = debuginfo_level_rustc_toml;
            debuginfo_level_std = debuginfo_level_std_toml;
            debuginfo_level_tools = debuginfo_level_tools_toml;
            debuginfo_level_tests = debuginfo_level_tests_toml;
            lld_enabled = lld_enabled_toml;
            std_features = std_features_toml;
            optimize = optimize_toml;
            config.rust_new_symbol_mangling = new_symbol_mangling;
            set(&mut config.rust_optimize_tests, optimize_tests);
            set(&mut config.codegen_tests, codegen_tests);
            set(&mut config.rust_rpath, rpath);
            set(&mut config.rust_strip, strip);
            set(&mut config.rust_frame_pointers, frame_pointers);
            config.rust_stack_protector = stack_protector;
            set(&mut config.jemalloc, jemalloc);
            set(&mut config.test_compare_mode, test_compare_mode);
            set(&mut config.backtrace, backtrace);
            config.description = description;
            set(&mut config.rust_dist_src, dist_src);
            set(&mut config.verbose_tests, verbose_tests);
            if let Some(true) = incremental {
                config.incremental = true;
            }
            set(&mut config.lld_mode, lld_mode);
            set(&mut config.llvm_bitcode_linker_enabled, llvm_bitcode_linker);
            config.rust_randomize_layout = randomize_layout.unwrap_or_default();
            config.llvm_tools_enabled = llvm_tools.unwrap_or(true);
            if parallel_compiler.is_some() {
                println!(
                    "WARNING: The `rust.parallel-compiler` option is deprecated and does nothing. The parallel compiler (with one thread) is now the default"
                );
            }
            config.llvm_enzyme = llvm_enzyme
                .unwrap_or(config.channel == "dev" || config.channel == "nightly");
            config.rustc_default_linker = default_linker;
            config.musl_root = musl_root.map(PathBuf::from);
            config.save_toolstates = save_toolstates.map(PathBuf::from);
            set(
                &mut config.deny_warnings,
                match flags.warnings {
                    Warnings::Deny => Some(true),
                    Warnings::Warn => Some(false),
                    Warnings::Default => deny_warnings,
                },
            );
            set(&mut config.backtrace_on_ice, backtrace_on_ice);
            set(&mut config.rust_verify_llvm_ir, verify_llvm_ir);
            config.rust_thin_lto_import_instr_limit = thin_lto_import_instr_limit;
            set(&mut config.rust_remap_debuginfo, remap_debuginfo);
            set(&mut config.control_flow_guard, control_flow_guard);
            set(&mut config.ehcont_guard, ehcont_guard);
            config.llvm_libunwind_default = llvm_libunwind
                .map(|v| v.parse().expect("failed to parse rust.llvm-libunwind"));
            if let Some(ref backends) = codegen_backends {
                let available_backends = ["llvm", "cranelift", "gcc"];
                config.rust_codegen_backends = backends
                    .iter()
                    .map(|s| {
                        if let Some(backend) = s.strip_prefix(CODEGEN_BACKEND_PREFIX) {
                            if available_backends.contains(&backend) {
                                panic!(
                                    "Invalid value '{s}' for 'rust.codegen-backends'. Instead, please use '{backend}'."
                                );
                            } else {
                                println!(
                                    "HELP: '{s}' for 'rust.codegen-backends' might fail. \
                                Codegen backends are mostly defined without the '{CODEGEN_BACKEND_PREFIX}' prefix. \
                                In this case, it would be referred to as '{backend}'."
                                );
                            }
                        }
                        s.clone()
                    })
                    .collect();
            }
            config.rust_codegen_units = codegen_units.map(threads_from_config);
            config.rust_codegen_units_std = codegen_units_std.map(threads_from_config);
            config.rust_profile_use = flags.rust_profile_use.or(profile_use);
            config.rust_profile_generate = flags
                .rust_profile_generate
                .or(profile_generate);
            config.rust_lto = lto
                .as_deref()
                .map(|value| RustcLto::from_str(value).unwrap())
                .unwrap_or_default();
            config.rust_validate_mir_opts = validate_mir_opts;
        } else {
            config.rust_profile_use = flags.rust_profile_use;
            config.rust_profile_generate = flags.rust_profile_generate;
        }
        config.reproducible_artifacts = flags.reproducible_artifact;
        if let Some(commit) = &config.download_rustc_commit {
            if is_user_configured_rust_channel {
                println!(
                    "WARNING: `rust.download-rustc` is enabled. The `rust.channel` option will be overridden by the CI rustc's channel."
                );
                let channel = config
                    .read_file_by_commit(&config.ci.channel_file, commit)
                    .trim()
                    .to_owned();
                config.channel = channel;
            }
        } else if config.rust_info.is_from_tarball() && !is_user_configured_rust_channel
        {
            ci_channel.clone_into(&mut config.channel);
        }
        if let Some(llvm) = toml.llvm {
            let Llvm {
                optimize: optimize_toml,
                thin_lto,
                release_debuginfo,
                assertions: _,
                tests,
                enzyme,
                plugins,
                ccache,
                static_libstdcpp,
                libzstd,
                ninja,
                targets,
                experimental_targets,
                link_jobs,
                link_shared,
                version_suffix,
                clang_cl,
                cflags,
                cxxflags,
                ldflags,
                use_libcxx,
                use_linker,
                allow_old_toolchain,
                offload,
                polly,
                clang,
                enable_warnings,
                download_ci_llvm,
                build_config,
                enable_projects,
            } = llvm;
            match ccache {
                Some(StringOrBool::String(ref s)) => config.ccache = Some(s.to_string()),
                Some(StringOrBool::Bool(true)) => {
                    config.ccache = Some("ccache".to_string());
                }
                Some(StringOrBool::Bool(false)) | None => {}
            }
            set(&mut config.ninja_in_file, ninja);
            llvm_tests = tests;
            llvm_enzyme = enzyme;
            llvm_offload = offload;
            llvm_plugins = plugins;
            set(&mut config.llvm_optimize, optimize_toml);
            set(&mut config.llvm_thin_lto, thin_lto);
            set(&mut config.llvm_release_debuginfo, release_debuginfo);
            set(&mut config.llvm_static_stdcpp, static_libstdcpp);
            set(&mut config.llvm_libzstd, libzstd);
            if let Some(v) = link_shared {
                config.llvm_link_shared.set(Some(v));
            }
            config.llvm_targets.clone_from(&targets);
            config.llvm_experimental_targets.clone_from(&experimental_targets);
            config.llvm_link_jobs = link_jobs;
            config.llvm_version_suffix.clone_from(&version_suffix);
            config.llvm_clang_cl.clone_from(&clang_cl);
            config.llvm_enable_projects.clone_from(&enable_projects);
            config.llvm_cflags.clone_from(&cflags);
            config.llvm_cxxflags.clone_from(&cxxflags);
            config.llvm_ldflags.clone_from(&ldflags);
            set(&mut config.llvm_use_libcxx, use_libcxx);
            config.llvm_use_linker.clone_from(&use_linker);
            config.llvm_allow_old_toolchain = allow_old_toolchain.unwrap_or(false);
            config.llvm_offload = offload.unwrap_or(false);
            config.llvm_polly = polly.unwrap_or(false);
            config.llvm_clang = clang.unwrap_or(false);
            config.llvm_enable_warnings = enable_warnings.unwrap_or(false);
            config.llvm_build_config = build_config
                .clone()
                .unwrap_or(Default::default());
            config.llvm_from_ci = config
                .parse_download_ci_llvm(download_ci_llvm, config.llvm_assertions);
            if config.llvm_from_ci {
                let warn = |option: &str| {
                    println!(
                        "WARNING: `{option}` will only be used on `compiler/rustc_llvm` build, not for the LLVM build."
                    );
                    println!(
                        "HELP: To use `{option}` for LLVM builds, set `download-ci-llvm` option to false."
                    );
                };
                if static_libstdcpp.is_some() {
                    warn("static-libstdcpp");
                }
                if link_shared.is_some() {
                    warn("link-shared");
                }
                if libzstd.is_some() {
                    println!(
                        "WARNING: when using `download-ci-llvm`, the local `llvm.libzstd` option, \
                        like almost all `llvm.*` options, will be ignored and set by the LLVM CI \
                        artifacts builder config."
                    );
                    println!(
                        "HELP: To use `llvm.libzstd` for LLVM/LLD builds, set `download-ci-llvm` option to false."
                    );
                }
            }
            if !config.llvm_from_ci && config.llvm_thin_lto && link_shared.is_none() {
                config.llvm_link_shared.set(Some(true));
            }
        } else {
            config.llvm_from_ci = config.parse_download_ci_llvm(None, false);
        }
        if let Some(t) = toml.target {
            for (triple, cfg) in t {
                let mut target = Target::from_triple(&triple);
                if let Some(ref s) = cfg.llvm_config {
                    if config.download_rustc_commit.is_some()
                        && triple == *config.build.triple
                    {
                        panic!(
                            "setting llvm_config for the host is incompatible with download-rustc"
                        );
                    }
                    target.llvm_config = Some(config.src.join(s));
                }
                if let Some(patches) = cfg.llvm_has_rust_patches {
                    assert!(
                        config.submodules == Some(false) || cfg.llvm_config.is_some(),
                        "use of `llvm-has-rust-patches` is restricted to cases where either submodules are disabled or llvm-config been provided"
                    );
                    target.llvm_has_rust_patches = Some(patches);
                }
                if let Some(ref s) = cfg.llvm_filecheck {
                    target.llvm_filecheck = Some(config.src.join(s));
                }
                target.llvm_libunwind = cfg
                    .llvm_libunwind
                    .as_ref()
                    .map(|v| {
                        v.parse()
                            .unwrap_or_else(|_| {
                                panic!("failed to parse target.{triple}.llvm-libunwind")
                            })
                    });
                if let Some(s) = cfg.no_std {
                    target.no_std = s;
                }
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
                if let Some(ref backends) = cfg.codegen_backends {
                    let available_backends = ["llvm", "cranelift", "gcc"];
                    target.codegen_backends = Some(
                        backends
                            .iter()
                            .map(|s| {
                                if let Some(backend) = s
                                    .strip_prefix(CODEGEN_BACKEND_PREFIX)
                                {
                                    if available_backends.contains(&backend) {
                                        panic!(
                                            "Invalid value '{s}' for 'target.{triple}.codegen-backends'. Instead, please use '{backend}'."
                                        );
                                    } else {
                                        println!(
                                            "HELP: '{s}' for 'target.{triple}.codegen-backends' might fail. \
                                    Codegen backends are mostly defined without the '{CODEGEN_BACKEND_PREFIX}' prefix. \
                                    In this case, it would be referred to as '{backend}'."
                                        );
                                    }
                                }
                                s.clone()
                            })
                            .collect(),
                    );
                }
                target.split_debuginfo = cfg
                    .split_debuginfo
                    .as_ref()
                    .map(|v| {
                        v.parse()
                            .unwrap_or_else(|_| {
                                panic!("invalid value for target.{triple}.split-debuginfo")
                            })
                    });
                config.target_config.insert(TargetSelection::from_user(&triple), target);
            }
        }
        if config.llvm_from_ci {
            let triple = &config.build.triple;
            let ci_llvm_bin = config.ci_llvm_root().join("bin");
            let build_target = config
                .target_config
                .entry(config.build)
                .or_insert_with(|| Target::from_triple(triple));
            check_ci_llvm!(build_target.llvm_config);
            check_ci_llvm!(build_target.llvm_filecheck);
            build_target.llvm_config = Some(
                ci_llvm_bin.join(exe("llvm-config", config.build)),
            );
            build_target.llvm_filecheck = Some(
                ci_llvm_bin.join(exe("FileCheck", config.build)),
            );
        }
        if let Some(dist) = toml.dist {
            let Dist {
                sign_folder,
                upload_addr,
                src_tarball,
                compression_formats,
                compression_profile,
                include_mingw_linker,
                vendor,
            } = dist;
            config.dist_sign_folder = sign_folder.map(PathBuf::from);
            config.dist_upload_addr = upload_addr;
            config.dist_compression_formats = compression_formats;
            set(&mut config.dist_compression_profile, compression_profile);
            set(&mut config.rust_dist_src, src_tarball);
            set(&mut config.dist_include_mingw_linker, include_mingw_linker);
            config.dist_vendor = vendor
                .unwrap_or_else(|| {
                    config.rust_info.is_managed_git_subrepository()
                        || config.rust_info.is_from_tarball()
                });
        }
        if let Some(r) = rustfmt {
            *config.initial_rustfmt.borrow_mut() = if r.exists() {
                RustfmtState::SystemToolchain(r)
            } else {
                RustfmtState::Unavailable
            };
        }
        config.llvm_tests = llvm_tests.unwrap_or(false);
        config.llvm_enzyme = llvm_enzyme.unwrap_or(false);
        config.llvm_offload = llvm_offload.unwrap_or(false);
        config.llvm_plugins = llvm_plugins.unwrap_or(false);
        config.rust_optimize = optimize.unwrap_or(RustOptimize::Bool(true));
        if config.build.triple == "x86_64-unknown-linux-gnu"
            && config.hosts == [config.build]
            && (config.channel == "dev" || config.channel == "nightly")
        {
            let no_llvm_config = config
                .target_config
                .get(&config.build)
                .is_some_and(|target_config| target_config.llvm_config.is_none());
            let enable_lld = config.llvm_from_ci || no_llvm_config;
            config.lld_enabled = lld_enabled.unwrap_or(enable_lld);
        } else {
            set(&mut config.lld_enabled, lld_enabled);
        }
        if matches!(config.lld_mode, LldMode::SelfContained) && !config.lld_enabled
            && flags.stage.unwrap_or(0) > 0
        {
            panic!(
                "Trying to use self-contained lld as a linker, but LLD is not being added to the sysroot. Enable it with rust.lld = true."
            );
        }
        let default_std_features = BTreeSet::from([String::from("panic-unwind")]);
        config.rust_std_features = std_features.unwrap_or(default_std_features);
        let default = debug == Some(true);
        config.rustc_debug_assertions = rustc_debug_assertions.unwrap_or(default);
        config.std_debug_assertions = std_debug_assertions
            .unwrap_or(config.rustc_debug_assertions);
        config.rust_overflow_checks = overflow_checks.unwrap_or(default);
        config.rust_overflow_checks_std = overflow_checks_std
            .unwrap_or(config.rust_overflow_checks);
        config.rust_debug_logging = debug_logging
            .unwrap_or(config.rustc_debug_assertions);
        let with_defaults = |debuginfo_level_specific: Option<_>| {
            debuginfo_level_specific
                .or(debuginfo_level)
                .unwrap_or(
                    if debug == Some(true) {
                        DebuginfoLevel::Limited
                    } else {
                        DebuginfoLevel::None
                    },
                )
        };
        config.rust_debuginfo_level_rustc = with_defaults(debuginfo_level_rustc);
        config.rust_debuginfo_level_std = with_defaults(debuginfo_level_std);
        config.rust_debuginfo_level_tools = with_defaults(debuginfo_level_tools);
        config.rust_debuginfo_level_tests = debuginfo_level_tests
            .unwrap_or(DebuginfoLevel::None);
        config.optimized_compiler_builtins = toml.build.as_ref().and_then(|b| b.optimized_compiler_builtins)
            .unwrap_or(config.channel != "dev");
        config.compiletest_diff_tool = toml.build.as_ref().and_then(|b| b.compiletest_diff_tool);
        let download_rustc = config.download_rustc_commit.is_some();
        config.stage = match config.cmd {
            Subcommand::Check { .. } => flags.stage.or(toml.build.as_ref().and_then(|b| b.check_stage)).unwrap_or(0),
            Subcommand::Doc { .. } => {
                flags.stage.or(toml.build.as_ref().and_then(|b| b.doc_stage)).unwrap_or(if download_rustc { 2 } else { 0 })
            }
            Subcommand::Build { .. } => {
                flags.stage.or(toml.build.as_ref().and_then(|b| b.build_stage)).unwrap_or(if download_rustc { 2 } else { 1 })
            }
            Subcommand::Test { .. } | Subcommand::Miri { .. } => {
                flags.stage.or(toml.build.as_ref().and_then(|b| b.test_stage)).unwrap_or(if download_rustc { 2 } else { 1 })
            }
            Subcommand::Bench { .. } => flags.stage.or(toml.build.as_ref().and_then(|b| b.bench_stage)).unwrap_or(2),
            Subcommand::Dist { .. } => flags.stage.or(toml.build.as_ref().and_then(|b| b.dist_stage)).unwrap_or(2),
            Subcommand::Install { .. } => flags.stage.or(toml.build.as_ref().and_then(|b| b.install_stage)).unwrap_or(2),
            Subcommand::Perf { .. } => flags.stage.unwrap_or(1),
            Subcommand::Clean { .. }
            | Subcommand::Clippy { .. }
            | Subcommand::Fix { .. }
            | Subcommand::Run { .. }
            | Subcommand::Setup { .. }
            | Subcommand::Format { .. }
            | Subcommand::Suggest { .. }
            | Subcommand::Vendor { .. } => flags.stage.unwrap_or(0),
        };
        #[cfg(not(test))]
        if flags.stage.is_none() && build_helper::ci::CiEnv::is_ci() {
            match config.cmd {
                Subcommand::Test { .. }
                | Subcommand::Miri { .. }
                | Subcommand::Doc { .. }
                | Subcommand::Build { .. }
                | Subcommand::Bench { .. }
                | Subcommand::Dist { .. }
                | Subcommand::Install { .. } => {
                    assert_eq!(
                        config.stage, 2,
                        "x.py should be run with `--stage 2` on CI, but was run with `--stage {}`",
                        config.stage,
                    );
                }
                Subcommand::Clean { .. }
                | Subcommand::Check { .. }
                | Subcommand::Clippy { .. }
                | Subcommand::Fix { .. }
                | Subcommand::Run { .. }
                | Subcommand::Setup { .. }
                | Subcommand::Format { .. }
                | Subcommand::Suggest { .. }
                | Subcommand::Vendor { .. }
                | Subcommand::Perf { .. } => {}
            }
        }
        config
    }
    /// Runs a command, printing out nice contextual information if it fails.
    /// Exits if the command failed to execute at all, otherwise returns its
    /// `status.success()`.
    pub(crate) fn test_args(&self) -> Vec<&str> {
        let mut test_args = match self.cmd {
            Subcommand::Test { ref test_args, .. }
            | Subcommand::Bench { ref test_args, .. }
            | Subcommand::Miri { ref test_args, .. } => {
                test_args.iter().flat_map(|s| s.split_whitespace()).collect()
            }
            _ => vec![],
        };
        test_args.extend(self.free_args.iter().map(|s| s.as_str()));
        test_args
    }
    pub(crate) fn args(&self) -> Vec<&str> {
        let mut args = match self.cmd {
            Subcommand::Run { ref args, .. } => {
                args.iter().flat_map(|s| s.split_whitespace()).collect()
            }
            _ => vec![],
        };
        args.extend(self.free_args.iter().map(|s| s.as_str()));
        args
    }
    /// Returns the content of the given file at a specific commit.
    pub(crate) fn read_file_by_commit(&self, file: &Path, commit: &str) -> String {
        assert!(
            self.rust_info.is_managed_git_subrepository(),
            "`Config::read_file_by_commit` is not supported in non-git sources."
        );
        let mut git = helpers::git(Some(&self.src));
        git.arg("show").arg(format!("{commit}:{}", file.to_str().unwrap()));
        output(git.as_command_mut())
    }
    /// Bootstrap embeds a version number into the name of shared libraries it uploads in CI.
    /// Return the version it would have used for the given commit.
    pub(crate) fn artifact_version_part(&self, commit: &str) -> String {
        let (channel, version) = if self.rust_info.is_managed_git_subrepository() {
            let channel = self
                .read_file_by_commit(&PathBuf::from("src/ci/channel"), commit)
                .trim()
                .to_owned();
            let version = self
                .read_file_by_commit(&self.ci.version_file, commit)
                .trim()
                .to_owned();
            (channel, version)
        } else {
            let channel = fs::read_to_string(&self.ci.channel_file);
            let version = fs::read_to_string(&self.ci.version_file);
            match (channel, version) {
                (Ok(channel), Ok(version)) => {
                    (channel.trim().to_owned(), version.trim().to_owned())
                }
                (channel, version) => {
                    let src = self.src.display();
                    eprintln!(
                        "ERROR: failed to determine artifact channel and/or version"
                    );
                    eprintln!(
                        "HELP: consider using a git checkout or ensure these files are readable"
                    );
                    if let Err(channel) = channel {
                        eprintln!("reading {src}/src/ci/channel failed: {channel:?}");
                    }
                    if let Err(version) = version {
                        eprintln!("reading {src}/src/version failed: {version:?}");
                    }
                    panic!();
                }
            }
        };
        match channel.as_str() {
            "stable" => version,
            "beta" => channel,
            "nightly" => channel,
            other => unreachable!("{:?} is not recognized as a valid channel", other),
        }
    }
    /// Try to find the relative path of `bindir`, otherwise return it in full.
    pub fn bindir_relative(&self) -> &Path {
        let bindir = &self.bindir;
        if bindir.is_absolute() {
            if let Some(prefix) = &self.prefix {
                if let Ok(stripped) = bindir.strip_prefix(prefix) {
                    return stripped;
                }
            }
        }
        bindir
    }
    /// Try to find the relative path of `libdir`.
    pub fn libdir_relative(&self) -> Option<&Path> {
        let libdir = self.libdir.as_ref()?;
        if libdir.is_relative() {
            Some(libdir)
        } else {
            libdir.strip_prefix(self.prefix.as_ref()?).ok()
        }
    }
    /// The absolute path to the downloaded LLVM artifacts.
    pub(crate) fn ci_llvm_root(&self) -> PathBuf {
        assert!(self.llvm_from_ci);
        self.out.join(self.build).join("ci-llvm")
    }
    /// Directory where the extracted `rustc-dev` component is stored.
    pub(crate) fn ci_rustc_dir(&self) -> PathBuf {
        assert!(self.download_rustc());
        self.out.join(self.build).join("ci-rustc")
    }
    /// Determine whether llvm should be linked dynamically.
    ///
    /// If `false`, llvm should be linked statically.
    /// This is computed on demand since LLVM might have to first be downloaded from CI.
    pub(crate) fn llvm_link_shared(&self) -> bool {
        let mut opt = self.llvm_link_shared.get();
        if opt.is_none() && self.dry_run {
            return false;
        }
        let llvm_link_shared = *opt
            .get_or_insert_with(|| {
                if self.llvm_from_ci {
                    self.maybe_download_ci_llvm();
                    let ci_llvm = self.ci_llvm_root();
                    let link_type = t!(
                        std::fs::read_to_string(ci_llvm.join("link-type.txt")),
                        format!("CI llvm missing: {}", ci_llvm.display())
                    );
                    link_type == "dynamic"
                } else {
                    false
                }
            });
        self.llvm_link_shared.set(opt);
        llvm_link_shared
    }
    /// Return whether we will use a downloaded, pre-compiled version of rustc, or just build from source.
    pub(crate) fn download_rustc(&self) -> bool {
        self.download_rustc_commit().is_some()
    }
    pub(crate) fn download_rustc_commit(&self) -> Option<&str> {
        static DOWNLOAD_RUSTC: OnceLock<Option<String>> = OnceLock::new();
        if self.dry_run && DOWNLOAD_RUSTC.get().is_none() {
            return self.download_rustc_commit.as_deref();
        }
        DOWNLOAD_RUSTC
            .get_or_init(|| match &self.download_rustc_commit {
                None => None,
                Some(commit) => {
                    self.download_ci_rustc(commit);
                    if !self.llvm_from_ci {
                        if CiEnv::is_ci() {
                            println!(
                                "WARNING: LLVM submodule has changes, `download-rustc` will be disabled."
                            );
                            return None;
                        } else {
                            panic!(
                                "ERROR: LLVM submodule has changes, `download-rustc` can't be used."
                            );
                        }
                    }
                    if let Some(config_path) = &self.config {
                        let ci_config_toml = match get_builder_toml::get_builder_toml(
                            self,
                            "ci-rustc",
                        ) {
                            Ok(ci_config_toml) => ci_config_toml,
                            Err(e) if e.to_string().contains("unknown field") => {
                                println!(
                                    "WARNING: CI rustc has some fields that are no longer supported in bootstrap; download-rustc will be disabled."
                                );
                                println!(
                                    "HELP: Consider rebasing to a newer commit if available."
                                );
                                return None;
                            }
                            Err(e) => {
                                eprintln!(
                                    "ERROR: Failed to parse CI rustc config.toml: {e}"
                                );
                                exit!(2);
                            }
                        };
                        let current_config_toml = get_toml::get_toml(config_path)
                            .unwrap();
                        let res = check_incompatible_options_for_ci_rustc(
                            current_config_toml,
                            ci_config_toml,
                        );
                        let disable_ci_rustc_if_incompatible = env::var_os(
                                "DISABLE_CI_RUSTC_IF_INCOMPATIBLE",
                            )
                            .is_some_and(|s| s == "1" || s == "true");
                        if disable_ci_rustc_if_incompatible && res.is_err() {
                            println!(
                                "WARNING: download-rustc is disabled with `DISABLE_CI_RUSTC_IF_INCOMPATIBLE` env."
                            );
                            return None;
                        }
                        res.unwrap();
                    }
                    Some(commit.clone())
                }
            })
            .as_deref()
    }
    pub(crate) fn initial_rustfmt(&self) -> Option<PathBuf> {
        match &mut *self.initial_rustfmt.borrow_mut() {
            RustfmtState::SystemToolchain(p) | RustfmtState::Downloaded(p) => {
                Some(p.clone())
            }
            RustfmtState::Unavailable => None,
            r @ RustfmtState::LazyEvaluated => {
                if self.dry_run {
                    return Some(PathBuf::new());
                }
                let path = self.maybe_download_rustfmt();
                *r = if let Some(p) = &path {
                    RustfmtState::Downloaded(p.clone())
                } else {
                    RustfmtState::Unavailable
                };
                path
            }
        }
    }
    /// Runs a function if verbosity is greater than 0
    pub fn verbose(&self, f: impl Fn()) {
        if self.is_verbose() {
            f()
        }
    }
    pub fn sanitizers_enabled(&self, target: TargetSelection) -> bool {
        self.target_config
            .get(&target)
            .and_then(|t| t.sanitizers)
            .unwrap_or(self.sanitizers)
    }
    pub fn needs_sanitizer_runtime_built(&self, target: TargetSelection) -> bool {
        !target.is_msvc() && self.sanitizers_enabled(target)
    }
    pub fn any_sanitizers_to_build(&self) -> bool {
        self.target_config
            .iter()
            .any(|(ts, t)| !ts.is_msvc() && t.sanitizers.unwrap_or(self.sanitizers))
    }
    pub fn profiler_path(&self, target: TargetSelection) -> Option<&str> {
        match self.target_config.get(&target)?.profiler.as_ref()? {
            StringOrBool::String(s) => Some(s),
            StringOrBool::Bool(_) => None,
        }
    }
    pub fn profiler_enabled(&self, target: TargetSelection) -> bool {
        self.target_config
            .get(&target)
            .and_then(|t| t.profiler.as_ref())
            .map(StringOrBool::is_string_or_true)
            .unwrap_or(self.profiler)
    }
    pub fn any_profiler_enabled(&self) -> bool {
        self
            .target_config
            .values()
            .any(|t| matches!(& t.profiler, Some(p) if p.is_string_or_true()))
            || self.profiler
    }
    pub fn rpath_enabled(&self, target: TargetSelection) -> bool {
        self.target_config.get(&target).and_then(|t| t.rpath).unwrap_or(self.rust_rpath)
    }
    pub fn llvm_enabled(&self, target: TargetSelection) -> bool {
        self.codegen_backends(target).contains(&"llvm".to_owned())
    }
    pub fn llvm_libunwind(&self, target: TargetSelection) -> LlvmLibunwind {
        self.target_config
            .get(&target)
            .and_then(|t| t.llvm_libunwind)
            .or(self.llvm_libunwind_default)
            .unwrap_or(
                if target.contains("fuchsia") {
                    LlvmLibunwind::InTree
                } else {
                    LlvmLibunwind::No
                },
            )
    }
    pub fn split_debuginfo(&self, target: TargetSelection) -> SplitDebuginfo {
        self.target_config
            .get(&target)
            .and_then(|t| t.split_debuginfo)
            .unwrap_or_else(|| SplitDebuginfo::default_for_platform(target))
    }
    /// Returns whether or not submodules should be managed by bootstrap.
    pub fn submodules(&self) -> bool {
        self.submodules.unwrap_or(self.rust_info.is_managed_git_subrepository())
    }
    pub fn codegen_backends(&self, target: TargetSelection) -> &[String] {
        self.target_config
            .get(&target)
            .and_then(|cfg| cfg.codegen_backends.as_deref())
            .unwrap_or(&self.rust_codegen_backends)
    }
    pub fn default_codegen_backend(&self, target: TargetSelection) -> Option<String> {
        self.codegen_backends(target).first().cloned()
    }
    pub fn git_config(&self) -> GitConfig<'_> {
        GitConfig {
            git_repository: &self.stage0_metadata.config.git_repository,
            nightly_branch: &self.stage0_metadata.config.nightly_branch,
            git_merge_commit_email: &self.stage0_metadata.config.git_merge_commit_email,
        }
    }
    /// Given a path to the directory of a submodule, update it.
    ///
    /// `relative_path` should be relative to the root of the git repository, not an absolute path.
    ///
    /// This *does not* update the submodule if `config.toml` explicitly says
    /// not to, or if we're not in a git repository (like a plain source
    /// tarball). Typically [`crate::Build::require_submodule`] should be
    /// used instead to provide a nice error to the user if the submodule is
    /// missing.
    pub(crate) fn update_submodule(&self, relative_path: &str) {
        if !self.submodules() {
            return;
        }
        let absolute_path = self.src.join(relative_path);
        if !GitInfo::new(false, &absolute_path).is_managed_git_subrepository()
            && !helpers::dir_is_empty(&absolute_path)
        {
            return;
        }
        let submodule_git = || {
            let mut cmd = helpers::git(Some(&absolute_path));
            cmd.run_always();
            cmd
        };
        let checked_out_hash = output(
            submodule_git().args(["rev-parse", "HEAD"]).as_command_mut(),
        );
        let checked_out_hash = checked_out_hash.trim_end();
        let recorded = output(
            helpers::git(Some(&self.src))
                .run_always()
                .args(["ls-tree", "HEAD"])
                .arg(relative_path)
                .as_command_mut(),
        );
        let actual_hash = recorded
            .split_whitespace()
            .nth(2)
            .unwrap_or_else(|| panic!("unexpected output `{}`", recorded));
        if actual_hash == checked_out_hash {
            return;
        }
        println!("Updating submodule {relative_path}");
        self.check_run(
            helpers::git(Some(&self.src))
                .run_always()
                .args(["submodule", "-q", "sync"])
                .arg(relative_path),
        );
        let update = |progress: bool| {
            let current_branch = output_result(
                    helpers::git(Some(&self.src))
                        .allow_failure()
                        .run_always()
                        .args(["symbolic-ref", "--short", "HEAD"])
                        .as_command_mut(),
                )
                .map(|b| b.trim().to_owned());
            let mut git = helpers::git(Some(&self.src)).allow_failure();
            git.run_always();
            if let Ok(branch) = current_branch {
                let branch = branch.strip_prefix("heads/").unwrap_or(&branch);
                git.arg("-c").arg(format!("branch.{branch}.remote=origin"));
            }
            git.args(["submodule", "update", "--init", "--recursive", "--depth=1"]);
            if progress {
                git.arg("--progress");
            }
            git.arg(relative_path);
            git
        };
        if !self.check_run(&mut update(true)) {
            self.check_run(&mut update(false));
        }
        let has_local_modifications = !self
            .check_run(
                submodule_git().allow_failure().args(["diff-index", "--quiet", "HEAD"]),
            );
        if has_local_modifications {
            self.check_run(submodule_git().args(["stash", "push"]));
        }
        self.check_run(submodule_git().args(["reset", "-q", "--hard"]));
        self.check_run(submodule_git().args(["clean", "-qdfx"]));
        if has_local_modifications {
            self.check_run(submodule_git().args(["stash", "pop"]));
        }
    }
    #[cfg(feature = "bootstrap-self-test")]
    pub fn check_stage0_version(
        &self,
        _program_path: &Path,
        _component_name: &'static str,
    ) {}
    /// check rustc/cargo version is same or lower with 1 apart from the building one
    #[cfg(not(feature = "bootstrap-self-test"))]
    pub fn check_stage0_version(
        &self,
        program_path: &Path,
        component_name: &'static str,
    ) {
        use build_helper::util::fail;
        if self.dry_run {
            return;
        }
        let stage0_output = output(Command::new(program_path).arg("--version"));
        let mut stage0_output = stage0_output.lines().next().unwrap().split(' ');
        let stage0_name = stage0_output.next().unwrap();
        if stage0_name != component_name {
            fail(
                &format!(
                    "Expected to find {component_name} at {} but it claims to be {stage0_name}",
                    program_path.display()
                ),
            );
        }
        let stage0_version = semver::Version::parse(
                stage0_output.next().unwrap().split('-').next().unwrap().trim(),
            )
            .unwrap();
        let source_version = semver::Version::parse(
                fs::read_to_string(self.src.join("src/version")).unwrap().trim(),
            )
            .unwrap();
        if !(source_version == stage0_version
            || (source_version.major == stage0_version.major
                && (source_version.minor == stage0_version.minor
                    || source_version.minor == stage0_version.minor + 1)))
        {
            let prev_version = format!(
                "{}.{}.x", source_version.major, source_version.minor - 1
            );
            fail(
                &format!(
                    "Unexpected {component_name} version: {stage0_version}, we should use {prev_version}/{source_version} to build source with {source_version}"
                ),
            );
        }
    }
    /// Returns the commit to download, or `None` if we shouldn't download CI artifacts.
    pub fn download_ci_rustc_commit(
        &self,
        download_rustc: Option<StringOrBool>,
        llvm_assertions: bool,
    ) -> Option<String> {
        if !crate::is_download_ci_available(&self.build.triple, llvm_assertions) {
            return None;
        }
        let if_unchanged = match download_rustc {
            None => self.rust_info.is_managed_git_subrepository(),
            Some(StringOrBool::Bool(false)) => return None,
            Some(StringOrBool::Bool(true)) => false,
            Some(StringOrBool::String(s)) if s == "if-unchanged" => {
                if !self.rust_info.is_managed_git_subrepository() {
                    println!(
                        "ERROR: `download-rustc=if-unchanged` is only compatible with Git managed sources."
                    );
                    exit!(1);
                }
                true
            }
            Some(StringOrBool::String(other)) => {
                panic!("unrecognized option for download-rustc: {other}")
            }
        };
        let mut allowed_paths = RUSTC_IF_UNCHANGED_ALLOWED_PATHS.to_vec();
        if !CiEnv::is_ci() {
            allowed_paths.push(":!library");
        }
        let commit = if self.rust_info.is_managed_git_subrepository() {
            match self
                .last_modified_commit(&allowed_paths, "download-rustc", if_unchanged)
            {
                Some(commit) => commit,
                None => {
                    if if_unchanged {
                        return None;
                    }
                    println!("ERROR: could not find commit hash for downloading rustc");
                    println!("HELP: maybe your repository history is too shallow?");
                    println!(
                        "HELP: consider setting `rust.download-rustc=false` in config.toml"
                    );
                    println!(
                        "HELP: or fetch enough history to include one upstream commit"
                    );
                    exit!(1);
                }
            }
        } else {
            channel::read_commit_info_file(&self.src)
                .map(|info| info.sha.trim().to_owned())
                .expect("git-commit-info is missing in the project root")
        };
        if CiEnv::is_ci()
            && {
                let head_sha = crate::output(
                    helpers::git(Some(&self.src))
                        .arg("rev-parse")
                        .arg("HEAD")
                        .as_command_mut(),
                );
                let head_sha = head_sha.trim();
                commit == head_sha
            }
        {
            eprintln!("CI rustc commit matches with HEAD and we are in CI.");
            eprintln!(
                "`rustc.download-ci` functionality will be skipped as artifacts are not available."
            );
            return None;
        }
        Some(commit)
    }
    pub fn parse_download_ci_llvm(
        &self,
        download_ci_llvm: Option<StringOrBool>,
        asserts: bool,
    ) -> bool {
        let download_ci_llvm = download_ci_llvm.unwrap_or(StringOrBool::Bool(true));
        let if_unchanged = || {
            if self.rust_info.is_from_tarball() {
                println!(
                    "ERROR: 'if-unchanged' is only compatible with Git managed sources."
                );
                exit!(1);
            }
            #[cfg(not(feature = "bootstrap-self-test"))]
            self.update_submodule("src/llvm-project");
            let has_changes = self
                .last_modified_commit(&["src/llvm-project"], "download-ci-llvm", true)
                .is_none();
            if has_changes { false } else { llvm::is_ci_llvm_available(self, asserts) }
        };
        match download_ci_llvm {
            StringOrBool::Bool(b) => {
                if !b && self.download_rustc_commit.is_some() {
                    panic!(
                        "`llvm.download-ci-llvm` cannot be set to `false` if `rust.download-rustc` is set to `true` or `if-unchanged`."
                    );
                }
                b && llvm::is_ci_llvm_available(self, asserts)
            }
            StringOrBool::String(s) if s == "if-unchanged" => if_unchanged(),
            StringOrBool::String(other) => {
                panic!("unrecognized option for download-ci-llvm: {:?}", other)
            }
        }
    }
    /// Returns the last commit in which any of `modified_paths` were changed,
    /// or `None` if there are untracked changes in the working directory and `if_unchanged` is true.
    pub fn last_modified_commit(
        &self,
        modified_paths: &[&str],
        option_name: &str,
        if_unchanged: bool,
    ) -> Option<String> {
        assert!(
            self.rust_info.is_managed_git_subrepository(),
            "Can't run `Config::last_modified_commit` on a non-git source."
        );
        let commit = get_closest_merge_commit(Some(&self.src), &self.git_config(), &[])
            .unwrap();
        if commit.is_empty() {
            println!(
                "error: could not find commit hash for downloading components from CI"
            );
            println!("help: maybe your repository history is too shallow?");
            println!("help: consider disabling `{option_name}`");
            println!("help: or fetch enough history to include one upstream commit");
            exit!(1);
        }
        let mut git = helpers::git(Some(&self.src));
        git.args(["diff-index", "--quiet", &commit, "--"]).args(modified_paths);
        let has_changes = t!(git.as_command_mut().status()).success();
        if has_changes {
            if if_unchanged {
                if self.is_verbose() {
                    println!(
                        "warning: saw changes to one of {modified_paths:?} since {commit}; \
                            ignoring `{option_name}`"
                    );
                }
                return None;
            }
            println!(
                "warning: `{option_name}` is enabled, but there are changes to one of {modified_paths:?}"
            );
        }
        Some(commit.to_string())
    }
}
