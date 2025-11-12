//use crate::flags::Color;
//use crate::subcommand::Subcommand;
//use crate::DebuginfoLevel;

use crate::TomlConfig;
use crate::rust::Rust;
//use crate::Flags;


/// Compares the current Rust options against those in the CI rustc builder and detects any incompatible options.
/// It does this by destructuring the `Rust` instance to make sure every `Rust` field is covered and not missing.
pub fn check_incompatible_options_for_ci_rustc(
    current_config_toml: TomlConfig,
    ci_config_toml: TomlConfig,
) -> Result<(), String> {
    macro_rules! err {
        ($current:expr, $expected:expr) => {
            if let Some(current) = &$current { if Some(current) != $expected .as_ref() {
            return
            Err(format!("ERROR: Setting `rust.{}` is incompatible with `rust.download-rustc`. \
                        Current value: {:?}, Expected value(s): {}{:?}",
            stringify!($expected) .replace("_", "-"), $current, if $expected .is_some() {
            "None/" } else { "" }, $expected,)); }; };
        };
    }
    macro_rules! warn {
        ($current:expr, $expected:expr) => {
            if let Some(current) = &$current { if Some(current) != $expected .as_ref() {
            println!("WARNING: `rust.{}` has no effect with `rust.download-rustc`. \
                        Current value: {:?}, Expected value(s): {}{:?}",
            stringify!($expected) .replace("_", "-"), $current, if $expected .is_some() {
            "None/" } else { "" }, $expected,); }; };
        };
    }
    let (Some(current_rust_config), Some(ci_rust_config)) = (
        current_config_toml.rust,
        ci_config_toml.rust,
    ) else {
        return Ok(());
    };
    let Rust {
        optimize,
        randomize_layout,
        debug_logging,
        llvm_tools,
        llvm_bitcode_linker,
        lto,
        stack_protector,
        strip,
        lld_mode,
        jemalloc,
        rpath,
        channel,
        description,
        incremental,
        default_linker,
        std_features,
        debug: _,
        codegen_units: _,
        codegen_units_std: _,
        rustc_debug_assertions: _,
        std_debug_assertions: _,
        overflow_checks: _,
        overflow_checks_std: _,
        backtrace: _,
        parallel_compiler: _,
        musl_root: _,
        verbose_tests: _,
        optimize_tests: _,
        codegen_tests: _,
        omit_git_hash: _,
        dist_src: _,
        save_toolstates: _,
        codegen_backends: _,
        lld: _,
        deny_warnings: _,
        backtrace_on_ice: _,
        verify_llvm_ir: _,
        thin_lto_import_instr_limit: _,
        remap_debuginfo: _,
        test_compare_mode: _,
        llvm_libunwind: _,
        control_flow_guard: _,
        ehcont_guard: _,
        new_symbol_mangling: _,
        profile_generate: _,
        profile_use: _,
        download_rustc: _,
        validate_mir_opts: _,
        frame_pointers: _,
    } = ci_rust_config;
    err!(current_rust_config.optimize, optimize);
    err!(current_rust_config.randomize_layout, randomize_layout);
    err!(current_rust_config.debug_logging, debug_logging);

    err!(current_rust_config.rpath, rpath);
    err!(current_rust_config.strip, strip);
    err!(current_rust_config.lld_mode, lld_mode);
    err!(current_rust_config.llvm_tools, llvm_tools);
    err!(current_rust_config.llvm_bitcode_linker, llvm_bitcode_linker);
    err!(current_rust_config.jemalloc, jemalloc);
    err!(current_rust_config.default_linker, default_linker);
    err!(current_rust_config.stack_protector, stack_protector);
    err!(current_rust_config.lto, lto);
    err!(current_rust_config.std_features, std_features);
    warn!(current_rust_config.channel, channel);
    warn!(current_rust_config.description, description);
    warn!(current_rust_config.incremental, incremental);
    Ok(())
}
pub fn set<T>(field: &mut T, val: Option<T>) {
    if let Some(v) = val {
        *field = v;
    }
}
pub fn threads_from_config(v: u32) -> u32 {
    match v {
        0 => {
            std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get)
                as u32
        }
        n => n,
    }
}
