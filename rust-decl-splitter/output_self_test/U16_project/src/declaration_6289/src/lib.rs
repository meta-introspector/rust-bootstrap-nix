#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

pub (crate) trait U16 { fn as_usize (self) -> usize ; fn low_u8 (self) -> u8 ; fn high_u8 (self) -> u8 ; }