

use serde_derive::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[derive(Clone)]
pub struct LocalTargetConfig {
    pub llvm_config: Option<std::path::PathBuf>,
    pub llvm_has_rust_patches: Option<bool>,
    pub llvm_filecheck: Option<std::path::PathBuf>,
    pub llvm_libunwind: Option<String>,
    pub no_std: Option<bool>,
    pub cc: Option<std::path::PathBuf>,
    pub cxx: Option<std::path::PathBuf>,
    pub ar: Option<std::path::PathBuf>,
    pub ranlib: Option<std::path::PathBuf>,
    pub linker: Option<std::path::PathBuf>,
    pub crt_static: Option<bool>,
    pub musl_root: Option<std::path::PathBuf>,
    pub musl_libdir: Option<std::path::PathBuf>,
    pub wasi_root: Option<std::path::PathBuf>,
    pub qemu_rootfs: Option<std::path::PathBuf>,
    pub runner: Option<Vec<String>>,
    pub sanitizers: Option<bool>,
    pub profiler: Option<bool>,
    pub rpath: Option<bool>,
    pub codegen_backends: Option<Vec<String>>,
    pub split_debuginfo: Option<String>,
}
