#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

pub (crate) trait U64 { fn as_usize (self) -> usize ; fn low_u8 (self) -> u8 ; fn low_u16 (self) -> u16 ; fn low_u32 (self) -> u32 ; fn high_u32 (self) -> u32 ; }