use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub enum DebuginfoLevel { # [default] None , LineDirectivesOnly , LineTablesOnly , Limited , Full , }