#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

# [doc (hidden)] const VARIANTS : & 'static [& 'static str] = & ["Success" , "Skipped" , "Failed"] ;