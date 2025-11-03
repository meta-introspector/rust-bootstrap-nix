#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

pub (crate) trait Usize { fn as_u8 (self) -> u8 ; fn as_u16 (self) -> u16 ; fn as_u32 (self) -> u32 ; fn as_u64 (self) -> u64 ; }