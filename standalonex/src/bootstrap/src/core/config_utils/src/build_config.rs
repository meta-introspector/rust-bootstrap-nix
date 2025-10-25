use crate::prelude::*


use crate::parsed_config::ParsedConfig;
use crate::local_toml_config::LocalTomlConfig;
use crate::config_applicator::ConfigApplicator;

pub struct BuildConfigApplicator;

impl ConfigApplicator for BuildConfigApplicator {
    fn apply_to_config(&self, config: &mut ParsedConfig, toml: &LocalTomlConfig) {
        let build_config = toml.build.clone().unwrap_or_default();

        config.jobs = config.jobs.or(build_config.jobs).or(Some(0));

        if let Some(file_build) = build_config.build {
            config.build_triple = Some(file_build);
        };

        // config.out_dir = flags.build_dir.or_else(|| build_config.build_dir.map(PathBuf::from)); // flags is not available here
        // NOTE: Bootstrap spawns various commands with different working directories.
        // To avoid writing to random places on the file system, `config.out` needs to be an absolute path.
        // if !config.out.is_absolute() {
        //     // `canonicalize` requires the path to already exist. Use our vendored copy of `absolute` instead.
        //     config.out = absolute(&config.out).expect("can't make empty path absolute");
        // }

        config.hosts = if let Some(file_host) = build_config.host {
            file_host
        } else {
            vec![config.build_triple.clone().unwrap_or_default()]
        };
        config.targets = if let Some(file_target) = build_config.target {
            file_target
        } else {
            config.hosts.clone()
        };

        config.nodejs = build_config.nodejs;
        config.npm = build_config.npm;
        config.gdb = build_config.gdb;
        config.lldb = build_config.lldb;
        config.python = build_config.python;
        config.reuse = build_config.reuse;
        config.submodules = build_config.submodules;
        config.android_ndk = build_config.android_ndk;
        config.bootstrap_cache_path = build_config.bootstrap_cache_path;
        config.low_priority = build_config.low_priority;
        config.compiler_docs = build_config.compiler_docs;
        config.library_docs_private_items = build_config.library_docs_private_items;
        config.docs_minification = build_config.docs_minification;
        config.docs = build_config.docs;
        config.locked_deps = build_config.locked_deps;
        config.vendor = build_config.vendor;
        config.full_bootstrap = build_config.full_bootstrap;
        config.extended = build_config.extended;
        config.tools = build_config.tools;
        config.verbose = build_config.verbose;
        config.sanitizers = build_config.sanitizers;
        config.profiler = build_config.profiler;
        config.cargo_native_static = build_config.cargo_native_static;
    }
}
