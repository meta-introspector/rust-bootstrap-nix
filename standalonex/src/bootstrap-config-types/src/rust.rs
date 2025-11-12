use std::collections::*;
use build_helper::prelude::*;
use crate::define_config;
use crate::RustOptimize;
//use crate::DebuginfoLevel;
use crate::LldMode;
use crate::StringOrBool;
define_config! {
    #[derive(Deserialize)]
    struct Rust {
    optimize : Option < RustOptimize > = "optimize", debug : Option < bool > = "debug",
    codegen_units : Option < u32 > = "codegen-units", codegen_units_std : Option < u32 >
    = "codegen-units-std", rustc_debug_assertions : Option < bool > = "debug-assertions",
	randomize_layout : Option < bool > = "randomize-layout",
	std_debug_assertions :	Option < bool > = "debug-assertions-std",
	overflow_checks : Option < bool > =  "overflow-checks",
	overflow_checks_std : Option < bool > = "overflow-checks-std",
	debug_logging : Option < bool > = "debug-logging",
//	debuginfo_level : Option <DebuginfoLevel > = "debuginfo-level",
//	debuginfo_level_rustc : Option < DebuginfoLevel    > = "debuginfo-level-rustc", debuginfo_level_std : Option < DebuginfoLevel > =
//    "debuginfo-level-std", debuginfo_level_tools : Option < DebuginfoLevel > =
//    "debuginfo-level-tools", debuginfo_level_tests : Option < DebuginfoLevel > =
	//    "debuginfo-level-tests",
	backtrace : Option < bool > = "backtrace", incremental :
    Option < bool > = "incremental", parallel_compiler : Option < bool > =
    "parallel-compiler", default_linker : Option < String > = "default-linker", channel :
    Option < String > = "channel", description : Option < String > = "description",
    musl_root : Option < String > = "musl-root", rpath : Option < bool > = "rpath", strip
    : Option < bool > = "strip", frame_pointers : Option < bool > = "frame-pointers",
    stack_protector : Option < String > = "stack-protector", verbose_tests : Option <
    bool > = "verbose-tests", optimize_tests : Option < bool > = "optimize-tests",
    codegen_tests : Option < bool > = "codegen-tests", omit_git_hash : Option < bool > =
    "omit-git-hash", dist_src : Option < bool > = "dist-src", save_toolstates : Option <
    String > = "save-toolstates", codegen_backends : Option < Vec < String >> =
    "codegen-backends", llvm_bitcode_linker : Option < bool > = "llvm-bitcode-linker",
    lld : Option < bool > = "lld", lld_mode : Option < LldMode > = "use-lld", llvm_tools
    : Option < bool > = "llvm-tools", deny_warnings : Option < bool > = "deny-warnings",
    backtrace_on_ice : Option < bool > = "backtrace-on-ice", verify_llvm_ir : Option <
    bool > = "verify-llvm-ir", thin_lto_import_instr_limit : Option < u32 > =
    "thin-lto-import-instr-limit", remap_debuginfo : Option < bool > = "remap-debuginfo",
    jemalloc : Option < bool > = "jemalloc", test_compare_mode : Option < bool > =
    "test-compare-mode", llvm_libunwind : Option < String > = "llvm-libunwind",
    control_flow_guard : Option < bool > = "control-flow-guard", ehcont_guard : Option <
    bool > = "ehcont-guard", new_symbol_mangling : Option < bool > =
    "new-symbol-mangling", profile_generate : Option < String > = "profile-generate",
    profile_use : Option < String > = "profile-use", download_rustc : Option <
    StringOrBool > = "download-rustc", lto : Option < String > = "lto", validate_mir_opts
    : Option < u32 > = "validate-mir-opts", std_features : Option < BTreeSet < String >>
    = "std-features", }
}
