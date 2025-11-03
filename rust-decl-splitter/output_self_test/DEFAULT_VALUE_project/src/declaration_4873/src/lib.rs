#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

static DEFAULT_VALUE : :: std :: sync :: OnceLock < String > = :: std :: sync :: OnceLock :: new () ;