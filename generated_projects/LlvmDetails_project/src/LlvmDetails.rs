use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub enum LlvmDetails { Info (LlvmInfo) , Error (String) , Unknown , }