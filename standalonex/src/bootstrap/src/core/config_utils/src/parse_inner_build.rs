
use crate::parsed_config::ParsedConfig;
use crate::local_toml_config::LocalTomlConfig;
use crate::local_flags::LocalFlags;
use crate::local_build::LocalBuild;
use crate::dry_run::DryRun;

pub fn parse_inner_build(config: &mut ParsedConfig, toml: &mut LocalTomlConfig, _flags: &LocalFlags) {
    let LocalBuild {
        build: _,
        host: _,
        target: _,
        build_dir: _,
        cargo: _,
        rustc: _,
        rustfmt: _,
        cargo_clippy,
        docs: _,
        compiler_docs: _,
        library_docs_private_items: _,
        docs_minification: _,
        submodules,
        gdb,
        lldb,
        nodejs,
        npm,
        python,
        reuse,
        locked_deps: _,
        vendor: _,
        full_bootstrap: _,
        bootstrap_cache_path,
        extended: _,
        tools,
        verbose: _,
        sanitizers: _,
        profiler: _,
        cargo_native_static: _,
        low_priority: _,
        configure_args: _,
        local_rebuild: _,
        print_step_timings: _,
        print_step_rusage: _,
        check_stage: _,
        doc_stage: _,
        build_stage: _,
        test_stage: _,
        install_stage: _,
        dist_stage: _,
        bench_stage: _,
        patch_binaries_for_nix,
        // This field is only used by bootstrap.py
        metrics: _,
        android_ndk,
        optimized_compiler_builtins: _,
        jobs: _,
        compiletest_diff_tool: _,
        src: _build_src_from_toml,
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
