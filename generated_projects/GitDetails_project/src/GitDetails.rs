use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub enum GitDetails { Info (GitInfo) , Error (String) , Unknown , }