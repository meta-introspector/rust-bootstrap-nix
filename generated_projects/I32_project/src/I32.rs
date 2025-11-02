use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub (crate) trait I32 { fn as_usize (self) -> usize ; fn to_bits (self) -> u32 ; fn from_bits (n : u32) -> i32 ; }