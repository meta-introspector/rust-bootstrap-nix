use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub (crate) trait U32 { fn as_usize (self) -> usize ; fn low_u8 (self) -> u8 ; fn low_u16 (self) -> u16 ; fn high_u16 (self) -> u16 ; }