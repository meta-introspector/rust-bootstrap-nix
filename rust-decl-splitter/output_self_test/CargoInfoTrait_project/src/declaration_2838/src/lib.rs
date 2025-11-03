#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

pub trait CargoInfoTrait : Send + Sync + Debug { fn package_name (& self) -> Option < & str > ; fn version (& self) -> Option < & str > ; }