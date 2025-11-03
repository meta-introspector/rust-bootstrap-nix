#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

# [doc = " The default set of crates for opt-dist to collect rustc profiles."] pub const RUSTC_PGO_CRATES : & [& str] = & ["externs" , "ctfe-stress-5" , "cargo-0.60.0" , "token-stream-stress" , "match-stress" , "tuple-stress" , "diesel-1.4.8" , "bitmaps-3.1.0" ,] ;