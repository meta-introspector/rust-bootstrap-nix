use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub trait Merge { fn merge (& mut self , other : Self , replace : ReplaceOpt) ; }