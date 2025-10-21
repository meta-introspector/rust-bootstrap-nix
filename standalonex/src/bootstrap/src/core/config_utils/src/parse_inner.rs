use crate::prelude::*;
use std::path::absolute;
use std::path::Path;
use std::path::PathBuf;
use std::env;
use crate::Config;
use crate::Flags;
use crate::TomlConfig;
use crate::get_toml;
use crate::DryRun;
use crate::CiConfig;
use crate::TargetSelection;
use crate::RustOptimize;
use crate::Build;
use crate::Install;
use crate::Rust;
use crate::Llvm;
use crate::Dist;
use crate::TargetSelectionList;
use crate::StringOrBool;
use crate::RustcLto;
use crate::DebuginfoLevel;
use crate::LlvmLibunwind;
use crate::SplitDebuginfo;
use crate::Subcommand;
use crate::Warnings;
use crate::GitInfo;
use crate::t;
use crate::exe;
use crate::output;
use crate::threads_from_config;
use crate::set;
use crate::check_incompatible_options_for_ci_rustc;
use crate::is_download_ci_available;
use crate::get_closest_merge_commit;
use crate::channel;
use crate::helpers;
use crate::CiEnv;
use crate::exit;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::fs;
use std::process::Command;
use std::str::FromStr;
use std::sync::OnceLock;
use std::cmp;
use serde::Deserialize;
use serde_derive::Deserialize;
use crate::parse_inner_flags;
use crate::parse_inner_out;
use crate::parse_inner_stage0;
use crate::parse_inner_toml;
use crate::parse_inner_out;

pub(crate) fn parse_inner(
    mut flags: Flags,
    get_toml: impl Fn(&Path) -> Result<TomlConfig, toml::de::Error>,
) -> Config {
    let mut config = Config::default_opts();

    // Set flags.
    parse_inner_flags(&mut config, &mut flags);

    // Infer the rest of the configuration.

    parse_inner_src(&mut config, &flags, &build_src_from_toml);

    parse_inner_out(&mut config);

    parse_inner_stage0(&mut config, &toml);

    let mut toml = parse_inner_toml(&mut config, &flags, get_toml);

    if cfg!(test) {
        // When configuring bootstrap for tests, make sure to set the rustc and Cargo to the
        // same ones used to call the tests (if custom ones are not defined in the toml). If we
        // don't do that, bootstrap will use its own detection logic to find a suitable rustc
        // and Cargo, which doesn't work when the caller is specìfying a custom local rustc or
        // Cargo in their config.toml.
        let build = toml.build.get_or_insert_with(Default::default);
        build.rustc = build.rustc.take().or(std::env::var_os("RUSTC").map(|p| p.into()));
        build.cargo = build.cargo.take().or(std::env::var_os("CARGO").map(|p| p.into()));
    }

    if let Some(include) = &toml.profile {
        // Allows creating alias for profile names, allowing
        // profiles to be renamed while maintaining back compatibility
        // Keep in sync with `profile_aliases` in bootstrap.py
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
        let included_toml = get_toml::get_toml(&include_path).unwrap_or_else(|e| {
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
            toml::from_str(option).and_then(|table: toml::Value| TomlConfig::deserialize(table))
        }

        let mut err = match get_table(option) {
            Ok(v) => {
                override_toml.merge(v, ReplaceOpt::ErrorOnDuplicate);
                continue;
            }
            Err(e) => e,
        };
        // We want to be able to set string values without quotes,
        // like in `configure.py`. Try adding quotes around the right hand side
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
        eprintln!("failed to parse override `{option}`: `{err}`");
        exit!(2)
    }
    toml.merge(override_toml, ReplaceOpt::Override);

    let build_src = toml.build.as_ref().and_then(|b| b.src.clone());

    let Ci {
        channel_file,
        version_file,
        tools_dir,
        llvm_project_dir,
        gcc_dir,
    } = toml.ci.unwrap_or_default();

    set(&mut config.ci.channel_file, channel_file.map(PathBuf::from));
    set(&mut config.ci.version_file, version_file.map(PathBuf::from));
    set(&mut config.ci.tools_dir, tools_dir.map(PathBuf::from));
    set(&mut config.ci.llvm_project_dir, llvm_project_dir.map(PathBuf::from));
    set(&mut config.ci.gcc_dir, gcc_dir.map(PathBuf::from));

    config.change_id = toml.change_id.inner;

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
        // This field is only used by bootstrap.py
        metrics: _,
        android_ndk,
        optimized_compiler_builtins,
        jobs,
        compiletest_diff_tool,
        src: build_src_from_toml,
    } = toml.build.unwrap_or_default();

    config.jobs = Some(threads_from_config(flags.jobs.unwrap_or(jobs.unwrap_or(0))));

    if let Some(file_build) = build {
        config.build = TargetSelection::from_user(&file_build);
    };

    set(&mut config.out, flags.build_dir.or_else(|| build_dir.map(PathBuf::from)));
    // NOTE: Bootstrap spawns various commands with different working directories.
    // To avoid writing to random places on the file system, `config.out` needs to be an absolute path.
    if !config.out.is_absolute() {
        // `canonicalize` requires the path to already exist. Use our vendored copy of `absolute` instead.
        config.out = absolute(&config.out).expect("can't make empty path absolute");
    }

    if cargo_clippy.is_some() && rustc.is_none() {
        println!(
            "WARNING: Using `build.cargo-clippy` without `build.rustc` usually fails due to toolchain conflict."
        );
    }

    config.initial_cargo_clippy = cargo_clippy;

    config.initial_rustc = if let Some(rustc) = rustc {
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

    config.initial_cargo = if let Some(cargo) = cargo {
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

    // NOTE: it's important this comes *after* we set `initial_rustc` just above.
    if config.dry_run {
        let dir = config.out.join("tmp-dry-run");
        t!(fs::create_dir_all(&dir));
        config.out = dir;
    }

    config.hosts = if let Some(TargetSelectionList(arg_host)) = flags.host {
        arg_host
    } else if let Some(file_host) = host {
        file_host.iter().map(|h| TargetSelection::from_user(h)).collect()
    } else {
        vec![config.build]
    };
    config.targets = if let Some(TargetSelectionList(arg_target)) = flags.target {
        arg_target
    } else if let Some(file_target) = target {
        file_target.iter().map(|h| TargetSelection::from_user(h)).collect()
    } else {
        // If target is *not* configured, then default to the host
        // toolchains.
        config.hosts.clone()
    };

    config.nodejs = nodejs.map(PathBuf::from);
    config.npm = npm.map(PathBuf::from);
    config.gdb = gdb.map(PathBuf::from);
    config.lldb = lldb.map(PathBuf::from);
    config.python = python.map(PathBuf::from);
    config.reuse = reuse.map(PathBuf::from);
    config.submodules = submodules;
    config.android_ndk = android_ndk;
    config.bootstrap_cache_path = bootstrap_cache_path;
    set(&mut config.low_priority, low_priority);
    set(&mut config.compiler_docs, compiler_docs);
    set(&mut config.library_docs_private_items, library_docs_private_items);
    set(&mut config.docs_minification, docs_minification);
    set(&mut config.docs, docs);
    set(&mut config.locked_deps, locked_deps);
    set(&mut config.vendor, vendor);
    set(&mut config.full_bootstrap, full_bootstrap);
    set(&mut config.extended, extended);
    config.tools = tools;
    set(&mut config.verbose, verbose);
    set(&mut config.sanitizers, sanitizers);
    set(&mut config.profiler, profiler);
    set(&mut config.cargo_native_static, cargo_native_static);
    set(&mut config.configure_args, configure_args);
    set(&mut config.local_rebuild, local_rebuild);
    set(&mut config.print_step_timings, print_step_timings);
    set(&mut config.print_step_rusage, print_step_rusage);
    config.patch_binaries_for_nix = patch_binaries_for_nix;

    config.verbose = cmp::max(config.verbose, flags.verbose as usize);

    // Verbose flag is a good default for `rust.verbose-tests`.
    config.verbose_tests = config.is_verbose();

    if let Some(install) = toml.install {
        let Install { prefix, sysconfdir, docdir, bindir, libdir, mandir, datadir } = install;
        config.prefix = prefix.map(PathBuf::from);
        config.sysconfdir = sysconfdir.map(PathBuf::from);
        config.datadir = datadir.map(PathBuf::from);
        config.docdir = docdir.map(PathBuf::from);
        // Handle bindir specifically, as it's not an Option in Config
        if let Some(b) = bindir {
            config.bindir = PathBuf::from(b);
        } else if let Some(p) = &config.prefix {
            config.bindir = p.join("bin");
        }
        config.libdir = libdir.map(PathBuf::from);
        config.mandir = mandir.map(PathBuf::from);
    }

    config.llvm_assertions =
        toml.llvm.as_ref().map_or(false, |llvm| llvm.assertions.unwrap_or(false));

    // Store off these values as options because if they're not provided
    // we'll infer default values for them later
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

    let is_user_configured_rust_channel =
        if let Some(channel) = toml.rust.as_ref().and_then(|r| r.channel.clone()) {
            config.channel = channel;
            true
        } else {
            false
        };

    let default = config.channel == "dev";
    config.omit_git_hash = toml.rust.as_ref().and_then(|r| r.omit_git_hash).unwrap_or(default);

    config.rust_info = GitInfo::new(config.omit_git_hash, &config.src); // config.src is still the overall source root
    config.cargo_info = GitInfo::new(config.omit_git_hash, &config.ci.tools_dir.join("cargo"));
    config.rust_analyzer_info =
        GitInfo::new(config.omit_git_hash, &config.ci.tools_dir.join("rust-analyzer"));
    config.clippy_info =
        GitInfo::new(config.omit_git_hash, &config.ci.tools_dir.join("clippy"));
    config.miri_info = GitInfo::new(config.omit_git_hash, &config.ci.tools_dir.join("miri"));
    config.rustfmt_info =
        GitInfo::new(config.omit_git_hash, &config.ci.tools_dir.join("rustfmt"));
    config.enzyme_info =
        GitInfo::new(config.omit_git_hash, &config.ci.tools_dir.join("enzyme"));
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
            channel: _, // already handled above
            description,
            musl_root,
            rpath,
            verbose_tests,
            optimize_tests,
            codegen_tests,
            omit_git_hash: _, // already handled above
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

        config.download_rustc_commit =
            config.download_ci_rustc_commit(download_rustc, config.llvm_assertions);

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
        // in the case "false" is set explicitly, do not overwrite the command line args
        if let Some(true) = incremental {
            config.incremental = true;
        }
        set(&mut config.lld_mode, lld_mode);
        set(&mut config.llvm_bitcode_linker_enabled, llvm_bitcode_linker);

        config.rust_randomize_layout = randomize_layout.unwrap_or_default();
        config.llvm_tools_enabled = llvm_tools.unwrap_or(true);

        // FIXME: Remove this option at the end of 2024.
        if parallel_compiler.is_some() {
            println!(
                "WARNING: The `rust.parallel-compiler` option is deprecated and does nothing. The parallel compiler (with one thread) is now the default"
            );
        }

        config.llvm_enzyme =
            llvm_enzyme.unwrap_or(config.channel == "dev" || config.channel == "nightly");
        config.rustc_default_linker = default_linker;
        config.musl_root = musl_root.map(PathBuf::from);
        config.save_toolstates = save_toolstates.map(PathBuf::from);
        set(&mut config.deny_warnings, match flags.warnings {
            Warnings::Deny => Some(true),
            Warnings::Warn => Some(false),
            Warnings::Default => deny_warnings,
        });
        set(&mut config.backtrace_on_ice, backtrace_on_ice);
        set(&mut config.rust_verify_llvm_ir, verify_llvm_ir);
        config.rust_thin_lto_import_instr_limit = thin_lto_import_instr_limit;
        set(&mut config.rust_remap_debuginfo, remap_debuginfo);
        set(&mut config.control_flow_guard, control_flow_guard);
        set(&mut config.ehcont_guard, ehcont_guard);
        config.llvm_libunwind_default =
            llvm_libunwind.map(|v| v.parse().expect("failed to parse rust.llvm-libunwind"));

        if let Some(ref backends) = codegen_backends {
            let available_backends = ["llvm", "cranelift", "gcc"];

            config.rust_codegen_backends = backends.iter().map(|s| {
                if let Some(backend) = s.strip_prefix(CODEGEN_BACKEND_PREFIX) {
                    if available_backends.contains(&backend) {
                        panic!("Invalid value '{s}' for 'rust.codegen-backends'. Instead, please use '{backend}'.");
                    } else {
                        println!(r"HELP: '{s}' for 'rust.codegen-backends' might fail. \                             Codegen backends are mostly defined without the '{CODEGEN_BACKEND_PREFIX}' prefix. \                             In this case, it would be referred to as '{backend}'.");
                    }
                }

                s.clone()
            }).collect();
        }

        config.rust_codegen_units = codegen_units.map(threads_from_config);
        config.rust_codegen_units_std = codegen_units_std.map(threads_from_config);
        config.rust_profile_use = flags.rust_profile_use.or(profile_use);
        config.rust_profile_generate = flags.rust_profile_generate.or(profile_generate);
        config.rust_lto =
            lto.as_deref().map(|value| RustcLto::from_str(value).unwrap()).unwrap_or_default();
        config.rust_validate_mir_opts = validate_mir_opts;
    } else {
        config.rust_profile_use = flags.rust_profile_use;
        config.rust_profile_generate = flags.rust_profile_generate;
    }

    config.reproducible_artifacts = flags.reproducible_artifact;

    // We need to override `rust.channel` if it's manually specified when using the CI rustc.
    // This is because if the compiler uses a different channel than the one specified in config.toml, 
    // tests may fail due to using a different channel than the one used by the compiler during tests.
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
    } else if config.rust_info.is_from_tarball() && !is_user_configured_rust_channel {
        ci_channel.clone_into(&mut config.channel);
    }

    if let Some(llvm) = toml.llvm {
        let Llvm {
            optimize: optimize_toml,
            thin_lto,
            release_debuginfo,
            assertions: _, // already handled above
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
            Some(StringOrBool::Bool(false)) | None => {} // No ccache
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
        config.llvm_build_config = build_config.clone().unwrap_or(Default::default());

        config.llvm_from_ci =
            config.parse_download_ci_llvm(download_ci_llvm, config.llvm_assertions);

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

            // FIXME(#129153): instead of all the ad-hoc `download-ci-llvm` checks that follow,
            // use the `builder-config` present in tarballs since #128822 to compare the local
            // config to the ones used to build the LLVM artifacts on CI, and only notify users
            // if they've chosen a different value.

            if libzstd.is_some() {
                println!(
                    r"WARNING: when using `download-ci-llvm`, the local `llvm.libzstd` option, \                     like almost all `llvm.*` options, will be ignored and set by the LLVM CI \                     artifacts builder config."
                );
                println!(
                    "HELP: To use `llvm.libzstd` for LLVM/LLD builds, set `download-ci-llvm` option to false."
                );
            }
        }

        if !config.llvm_from_ci && config.llvm_thin_lto && link_shared.is_none() {
            // If we're building with ThinLTO on, by default we want to link
            // to LLVM shared, to avoid re-doing ThinLTO (which happens in
            // the link step) with each stage.
            config.llvm_link_shared.set(Some(true));
        }
    } else {
        config.llvm_from_ci = config.parse_download_ci_llvm(None, false);
    }

    if let Some(t) = toml.target {
        for (triple, cfg) in t {
            let mut target = Target::from_triple(&triple);

            if let Some(ref s) = cfg.llvm_config {
                if config.download_rustc_commit.is_some() && triple == *config.build.triple {
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
            target.llvm_libunwind = cfg.llvm_libunwind.as_ref().map(|v| {
                v.parse().unwrap_or_else(|_| {
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

                target.codegen_backends = Some(backends.iter().map(|s| {
                    if let Some(backend) = s.strip_prefix(CODEGEN_BACKEND_PREFIX) {
                        if available_backends.contains(&backend) {
                            panic!("Invalid value '{s}' for 'target.{triple}.codegen-backends'. Instead, please use '{backend}'.");
                        } else {
                            println!(r"HELP: '{s}' for 'target.{triple}.codegen-backends' might fail. \                             Codegen backends are mostly defined without the '{CODEGEN_BACKEND_PREFIX}' prefix. \                             In this case, it would be referred to as '{backend}'.");
                        }
                    }

                    s.clone()
                }).collect());
            }

            target.split_debuginfo = cfg.split_debuginfo.as_ref().map(|v| {
                v.parse().unwrap_or_else(|_| {
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
        build_target.llvm_config = Some(ci_llvm_bin.join(exe("llvm-config", config.build)));
        build_target.llvm_filecheck = Some(ci_llvm_bin.join(exe("FileCheck", config.build)));
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
        config.dist_vendor = vendor.unwrap_or_else(|| {
            // If we're building from git or tarball sources, enable it by default.
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

    // Now that we've reached the end of our configuration, infer the
    // default values for all options that we haven't otherwise stored yet.

    config.llvm_tests = llvm_tests.unwrap_or(false);
    config.llvm_enzyme = llvm_enzyme.unwrap_or(false);
    config.llvm_offload = llvm_offload.unwrap_or(false);
    config.llvm_plugins = llvm_plugins.unwrap_or(false);
    config.rust_optimize = optimize.unwrap_or(RustOptimize::Bool(true));

    // We make `x86_64-unknown-linux-gnu` use the self-contained linker by default, so we will
    // build our internal lld and use it as the default linker, by setting the `rust.lld` config
    // to true by default:
    // - on the `x86_64-unknown-linux-gnu` target
    // - on the `dev` and `nightly` channels
    // - when building our in-tree llvm (i.e. the target has not set an `llvm-config`), so that
    //   we're also able to build the corresponding lld
    // - or when using an external llvm that's downloaded from CI, which also contains our prebuilt
    //   lld
    // - otherwise, we'd be using an external llvm, and lld would not necessarily available and
    //   thus, disabled
    // - similarly, lld will not be built nor used by default when explicitly asked not to, e.g. 
    //   when the config sets `rust.lld = false`
    if config.build.triple == "x86_64-unknown-linux-gnu"
        && config.hosts == [config.build]
        && (config.channel == "dev" || config.channel == "nightly")
    {
        let no_llvm_config = config
            .target_config
            .get(&config.build)
            .is_some_and(|target_config| target_config.llvm_config.is_none());
        let enable_lld = config.llvm_from_ci || no_llvm_config;
        // Prefer the config setting in case an explicit opt-out is needed.
        config.lld_enabled = lld_enabled.unwrap_or(enable_lld);
    } else {
        set(&mut config.lld_enabled, lld_enabled);
    }

    if matches!(config.lld_mode, LldMode::SelfContained)
        && !config.lld_enabled
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
    config.std_debug_assertions = std_debug_assertions.unwrap_or(config.rustc_debug_assertions);
    config.rust_overflow_checks = overflow_checks.unwrap_or(default);
    config.rust_overflow_checks_std =
        overflow_checks_std.unwrap_or(config.rust_overflow_checks);

    config.rust_debug_logging = debug_logging.unwrap_or(config.rustc_debug_assertions);

    let with_defaults = |debuginfo_level_specific: Option<_>| {
        debuginfo_level_specific.or(debuginfo_level).unwrap_or(if debug == Some(true) {
            DebuginfoLevel::Limited
        } else {
            DebuginfoLevel::None
        })
    };
    config.rust_debuginfo_level_rustc = with_defaults(debuginfo_level_rustc);
    config.rust_debuginfo_level_std = with_defaults(debuginfo_level_std);
    config.rust_debuginfo_level_tools = with_defaults(debuginfo_level_tools);
    config.rust_debuginfo_level_tests = debuginfo_level_tests.unwrap_or(DebuginfoLevel::None);
    config.optimized_compiler_builtins =
        optimized_compiler_builtins.unwrap_or(config.channel != "dev");
    config.compiletest_diff_tool = compiletest_diff_tool;

    let download_rustc = config.download_rustc_commit.is_some();
    // See https://github.com/rust-lang/compiler-team/issues/326
    config.stage = match config.cmd {
        Subcommand::Check { .. } => flags.stage.or(check_stage).unwrap_or(0),
        // `download-rustc` only has a speed-up for stage2 builds. Default to stage2 unless explicitly overridden.
        Subcommand::Doc { .. } => {
            flags.stage.or(doc_stage).unwrap_or(if download_rustc { 2 } else { 0 })
        }
        Subcommand::Build { .. } => {
            flags.stage.or(build_stage).unwrap_or(if download_rustc { 2 } else { 1 })
        }
        Subcommand::Test { .. } | Subcommand::Miri { .. } => {
            flags.stage.or(test_stage).unwrap_or(if download_rustc { 2 } else { 1 })
        }
        Subcommand::Bench { .. } => flags.stage.or(bench_stage).unwrap_or(2),
        Subcommand::Dist { .. } => flags.stage.or(dist_stage).unwrap_or(2),
        Subcommand::Install { .. } => flags.stage.or(install_stage).unwrap_or(2),
        Subcommand::Perf { .. } => flags.stage.unwrap_or(1),
        // These are all bootstrap tools, which don't depend on the compiler.
        // The stage we pass shouldn't matter, but use 0 just in case.
        Subcommand::Clean { .. }
        | Subcommand::Clippy { .. }
        | Subcommand::Fix { .. }
        | Subcommand::Run { .. }
        | Subcommand::Setup { .. }
        | Subcommand::Format { .. }
        | Subcommand::Suggest { .. }
        | Subcommand::Vendor { .. } => flags.stage.unwrap_or(0),
    };

    // CI should always run stage 2 builds, unless it specifically states otherwise
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
            | Subcommand::Perf { .. } => {} // These commands don't require stage 2
        }
    }

    config
}
