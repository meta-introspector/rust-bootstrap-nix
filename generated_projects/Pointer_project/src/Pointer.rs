use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub (crate) trait Pointer { fn as_usize (self) -> usize ; }