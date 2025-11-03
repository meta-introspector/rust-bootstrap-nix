#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

# [doc = " A trait that effectively gives us practical dynamic dispatch over anything"] # [doc = " that impls `Automaton`, but without needing to add a bunch of bounds to"] # [doc = " the core `Automaton` trait. Basically, we provide all of the marker traits"] # [doc = " that our automatons have, in addition to `Debug` impls and requiring that"] # [doc = " there is no borrowed data. Without these, the main `AhoCorasick` type would"] # [doc = " not be able to meaningfully impl `Debug` or the marker traits without also"] # [doc = " requiring that all impls of `Automaton` do so, which would be not great."] trait AcAutomaton : Automaton + Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static { }