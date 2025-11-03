#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

pub trait Merge { fn merge (& mut self , other : Self , replace : ReplaceOpt) ; }