use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct CollectedPreludeInfo { pub crate_name : String , pub crate_root : PathBuf , pub prelude_content : String , pub modified_files : Vec < PathBuf > , pub crate_root_modified : bool , pub file_processing_results : Vec < FileProcessingResult > , }