#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

pub (crate) trait I32 { fn as_usize (self) -> usize ; fn to_bits (self) -> u32 ; fn from_bits (n : u32) -> i32 ; }