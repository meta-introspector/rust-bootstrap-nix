#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

static SUBMODULES_PATHS : OnceLock < Vec < String > > = OnceLock :: new () ;