use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub enum CargoDetails { Info (CargoInfo) , Error (String) , Unknown , }