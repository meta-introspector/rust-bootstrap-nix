use crate::prelude::*;
define_config! {
    #[doc = " TOML representation of how each build target is configured."] struct
    TomlTarget { cc : Option < String > = "cc", cxx : Option < String > = "cxx", ar :
    Option < String > = "ar", ranlib : Option < String > = "ranlib", default_linker :
    Option < PathBuf > = "default-linker", linker : Option < String > = "linker",
    split_debuginfo : Option < String > = "split-debuginfo", llvm_config : Option <
    String > = "llvm-config", llvm_has_rust_patches : Option < bool > =
    "llvm-has-rust-patches", llvm_filecheck : Option < String > = "llvm-filecheck",
    llvm_libunwind : Option < String > = "llvm-libunwind", sanitizers : Option < bool > =
    "sanitizers", profiler : Option < StringOrBool > = "profiler", rpath : Option < bool
    > = "rpath", crt_static : Option < bool > = "crt-static", musl_root : Option < String
    > = "musl-root", musl_libdir : Option < String > = "musl-libdir", wasi_root : Option
    < String > = "wasi-root", qemu_rootfs : Option < String > = "qemu-rootfs", no_std :
    Option < bool > = "no-std", codegen_backends : Option < Vec < String >> =
    "codegen-backends", runner : Option < String > = "runner", }
}
