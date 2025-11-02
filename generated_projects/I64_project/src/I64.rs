use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub (crate) trait I64 { fn as_usize (self) -> usize ; fn to_bits (self) -> u64 ; fn from_bits (n : u64) -> i64 ; }