use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub fn set < T > (field : & mut T , val : Option < T >) { if let Some (v) = val { * field = v ; } }