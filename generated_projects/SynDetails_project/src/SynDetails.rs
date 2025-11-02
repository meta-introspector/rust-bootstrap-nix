use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub enum SynDetails { Info (SynInfo) , Error (String) , Unknown , }