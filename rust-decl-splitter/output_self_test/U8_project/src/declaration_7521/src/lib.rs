#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

pub (crate) trait U8 { fn as_usize (self) -> usize ; }