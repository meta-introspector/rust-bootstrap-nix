use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct LlvmConfig { # [serde (rename = "download-ci-llvm")] pub download_ci_llvm : Option < bool > , pub ninja : Option < bool > , }