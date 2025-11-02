use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct ErrorSample { pub file_path : PathBuf , pub rustc_version : String , pub rustc_host : String , pub error_message : String , pub error_type : String , pub code_snippet : Option < String > , pub timestamp : DateTime < Utc > , pub context : Option < String > , }