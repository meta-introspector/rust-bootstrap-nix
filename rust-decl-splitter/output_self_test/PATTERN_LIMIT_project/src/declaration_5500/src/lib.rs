#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

# [doc = " This is a limit placed on the total number of patterns we're willing to try"] # [doc = " and match at once. As more sophisticated algorithms are added, this number"] # [doc = " may be increased."] const PATTERN_LIMIT : usize = 128 ;