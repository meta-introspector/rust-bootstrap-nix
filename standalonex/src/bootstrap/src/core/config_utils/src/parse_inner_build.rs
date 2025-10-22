use std::path::PathBuf;
use crate::parsed_config::ParsedConfig;
use crate::local_toml_config::LocalTomlConfig;
use crate::local_flags::LocalFlags;
use crate::local_build::LocalBuild;
use crate::dry_run::DryRun;

pub fn parse_inner_build(config: &mut ParsedConfig, toml: &mut LocalTomlConfig, flags: &LocalFlags) {
    let LocalBuild {
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
    } = toml.build.clone().unwrap_or_default();

    config.initial_cargo_clippy = cargo_clippy;

    if config.dry_run != DryRun::Disabled {
        let dir = config.out.join("tmp-dry-run");
        config.out = dir;
    }

    config.nodejs = nodejs;
    config.npm = npm;
    config.gdb = gdb;
    config.lldb = lldb;
    config.python = python;
    config.reuse = reuse;
    config.submodules = submodules;
    config.android_ndk = android_ndk;
    config.bootstrap_cache_path = bootstrap_cache_path;
    config.tools = tools;
    config.patch_binaries_for_nix = patch_binaries_for_nix;
}
