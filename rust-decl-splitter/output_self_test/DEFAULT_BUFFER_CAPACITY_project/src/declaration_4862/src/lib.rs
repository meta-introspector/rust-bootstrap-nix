#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

# [doc = " The default buffer capacity that we use for the stream buffer."] const DEFAULT_BUFFER_CAPACITY : usize = 64 * (1 << 10) ;