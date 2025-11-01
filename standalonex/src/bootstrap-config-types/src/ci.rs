use build_helper::prelude::*;
define_config! {
    #[doc = " TOML representation of CI-related paths and settings."] #[derive(Default)]
    struct Ci { channel_file : Option < String > = "channel-file", version_file : Option
    < String > = "version-file", tools_dir : Option < String > = "tools-dir",
    llvm_project_dir : Option < String > = "llvm-project-dir", gcc_dir : Option < String
    > = "gcc-dir", }
}
