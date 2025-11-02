use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

fn is_complex_type (type_name : & str) -> bool { type_name == "syn" || type_name == "String" || type_name == "HashMap" || type_name == "PathBuf" || type_name == "clap" || type_name == "serde" }