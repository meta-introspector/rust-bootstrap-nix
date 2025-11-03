#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

pub (crate) trait Pointer { fn as_usize (self) -> usize ; }