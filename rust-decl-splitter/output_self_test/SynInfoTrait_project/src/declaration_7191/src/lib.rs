#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

pub trait SynInfoTrait : Send + Sync + Debug { fn parsed_type (& self) -> Option < & str > ; fn version (& self) -> Option < & str > ; }