use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub enum NixDetails { Info (NixInfo) , Error (String) , Unknown , }