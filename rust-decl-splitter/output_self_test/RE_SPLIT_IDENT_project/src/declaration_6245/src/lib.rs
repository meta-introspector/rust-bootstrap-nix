#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

static RE_SPLIT_IDENT : Lazy < Regex > = Lazy :: new (| | { Regex :: new (r"[^a-zA-Z0-9]+|(?<=[a-z])(?=[A-Z])|^_|_$") . unwrap () }) ;