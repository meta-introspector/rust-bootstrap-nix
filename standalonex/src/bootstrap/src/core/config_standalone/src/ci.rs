use config_macros::define_config;

define_config! {
    /// TOML representation of CI-related paths and settings.
    #[derive(Default)]
    struct Ci {
        channel_file: Option<String> = "channel-file",
        version_file: Option<String> = "version-file",
        tools_dir: Option<String> = "tools-dir",
        llvm_project_dir: Option<String> = "llvm-project-dir",
        gcc_dir: Option<String> = "gcc-dir",
    }
}
