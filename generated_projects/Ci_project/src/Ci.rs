use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " TOML representation of CI-related paths and settings."] pub struct Ci { pub channel_file : Option < String > , pub version_file : Option < String > , pub tools_dir : Option < String > , pub llvm_project_dir : Option < String > , pub gcc_dir : Option < String > , }