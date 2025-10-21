use std::path::absolute;
use bootstrap::Config;
use bootstrap::TomlConfig;
use bootstrap::Build;
use bootstrap::TargetSelection;
use crate::core::config::config_part2::{set, threads_from_config};
use bootstrap::Flags;
use bootstrap::TargetSelectionList;
use std::path::PathBuf;
use std::env;
use std::fs;
use crate::utils::helpers::{exe, t};

pub fn parse_inner_build(config: &mut Config, toml: &mut TomlConfig, flags: &Flags) {
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
}
