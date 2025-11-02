use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub enum LinuxDetails { Info (LinuxInfo) , Error (String) , Unknown , }