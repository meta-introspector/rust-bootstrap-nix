#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

static ACTIVE_GROUPS : Mutex < Vec < String > > = Mutex :: new (Vec :: new ()) ;