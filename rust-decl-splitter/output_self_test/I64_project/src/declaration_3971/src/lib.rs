#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

pub (crate) trait I64 { fn as_usize (self) -> usize ; fn to_bits (self) -> u64 ; fn from_bits (n : u64) -> i64 ; }