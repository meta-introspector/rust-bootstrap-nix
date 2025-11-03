#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

pub (crate) trait I8 { fn as_usize (self) -> usize ; fn to_bits (self) -> u8 ; fn from_bits (n : u8) -> i8 ; }