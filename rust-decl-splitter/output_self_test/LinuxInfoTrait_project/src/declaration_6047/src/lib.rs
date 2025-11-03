#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

pub trait LinuxInfoTrait : Send + Sync + Debug { fn kernel_version (& self) -> Option < & str > ; fn architecture (& self) -> Option < & str > ; }