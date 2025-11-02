use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub enum RustDetails { Info (RustDetailsInfo) , Error (String) , Unknown , }