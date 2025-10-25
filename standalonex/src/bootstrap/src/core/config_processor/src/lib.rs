use crate::prelude::*;


use bootstrap_config_utils::parsed_config::ParsedConfig;
use bootstrap_config_utils::dry_run::DryRun; // Import DryRun

// Placeholder for the actual bootstrap::Config struct
pub struct BootstrapConfig {
    pub dry_run: bool,
    pub out_dir: std::path::PathBuf,
    pub channel: Option<String>,
    pub jobs: Option<usize>,
    pub build_triple: Option<String>,
    pub rust_optimize_tests: Option<bool>,
    pub docs: Option<bool>,
    pub docs_minification: Option<bool>,
    pub rust_rpath: Option<bool>,
    pub rust_strip: Option<bool>,
    pub rust_dist_src: Option<bool>,
    pub deny_warnings: Option<bool>,
    pub dist_include_mingw_linker: Option<bool>,
    pub llvm_optimize: Option<bool>,
    pub llvm_static_stdcpp: Option<bool>,
    pub llvm_libzstd: Option<bool>,
    pub llvm_assertions: Option<bool>,
    pub llvm_tests: bool,
    pub llvm_enzyme_flag: Option<bool>,
    pub llvm_offload: bool,
    pub llvm_plugins: bool,
}

impl Default for BootstrapConfig {
    fn default() -> Self {
        BootstrapConfig {
            dry_run: false,
            out_dir: std::path::PathBuf::from("build"),
            channel: None,
            jobs: None,
            build_triple: None,
            rust_optimize_tests: None,
            docs: None,
            docs_minification: None,
            rust_rpath: None,
            rust_strip: None,
            rust_dist_src: None,
            deny_warnings: None,
            dist_include_mingw_linker: None,
            llvm_optimize: None,
            llvm_static_stdcpp: None,
            llvm_libzstd: None,
            llvm_assertions: None,
            llvm_tests: false,
            llvm_enzyme_flag: None,
            llvm_offload: false,
            llvm_plugins: false,
        }
    }
}

pub fn process_config(parsed_config: ParsedConfig) -> BootstrapConfig {
    BootstrapConfig {
        dry_run: parsed_config.dry_run != DryRun::Disabled,
        out_dir: parsed_config.out,
        channel: parsed_config.channel,
        jobs: parsed_config.jobs,
        build_triple: parsed_config.build_triple,
        rust_optimize_tests: parsed_config.rust_optimize_tests,
        docs: parsed_config.docs,
        docs_minification: parsed_config.docs_minification,
        rust_rpath: parsed_config.rust_rpath,
        rust_strip: parsed_config.rust_strip,
        rust_dist_src: parsed_config.rust_dist_src,
        deny_warnings: parsed_config.deny_warnings,
        dist_include_mingw_linker: parsed_config.dist_include_mingw_linker,
        llvm_optimize: parsed_config.llvm_optimize,
        llvm_static_stdcpp: parsed_config.llvm_static_stdcpp,
        llvm_libzstd: parsed_config.llvm_libzstd,
        llvm_assertions: parsed_config.llvm_assertions,
        llvm_tests: parsed_config.llvm_tests,
        llvm_enzyme_flag: parsed_config.llvm_enzyme_flag,
        llvm_offload: parsed_config.llvm_offload,
        llvm_plugins: parsed_config.llvm_plugins,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bootstrap_config_utils::parsed_config::ParsedConfig;
    use bootstrap_config_utils::dry_run::DryRun;
    use std::path::PathBuf;

    #[test]
    fn test_process_config() {
        let mut parsed_config = ParsedConfig::default();
        parsed_config.dry_run = DryRun::UserSelected;
        parsed_config.out = PathBuf::from("/tmp/test_output");
        parsed_config.channel = Some("nightly".to_string());
        parsed_config.jobs = Some(8);
        parsed_config.llvm_tests = true;
        parsed_config.llvm_enzyme_flag = Some(true);

        let bootstrap_config = process_config(parsed_config);

        assert_eq!(bootstrap_config.dry_run, true);
        assert_eq!(bootstrap_config.out_dir, PathBuf::from("/tmp/test_output"));
        assert_eq!(bootstrap_config.channel, Some("nightly".to_string()));
        assert_eq!(bootstrap_config.jobs, Some(8));
        assert_eq!(bootstrap_config.llvm_tests, true);
        assert_eq!(bootstrap_config.llvm_enzyme_flag, Some(true));
    }
}

