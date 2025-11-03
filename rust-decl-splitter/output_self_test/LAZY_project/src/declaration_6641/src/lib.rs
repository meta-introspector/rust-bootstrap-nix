#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

static LAZY : :: lazy_static :: lazy :: Lazy < Mutex < HashMap < String , FunctionMetrics > > , > = :: lazy_static :: lazy :: Lazy :: INIT ;