#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

# [doc = " A prefilter describes the behavior of fast literal scanners for quickly"] # [doc = " skipping past bytes in the haystack that we know cannot possibly"] # [doc = " participate in a match."] trait PrefilterI : Send + Sync + RefUnwindSafe + UnwindSafe + Debug + 'static { # [doc = " Returns the next possible match candidate. This may yield false"] # [doc = " positives, so callers must confirm a match starting at the position"] # [doc = " returned. This, however, must never produce false negatives. That is,"] # [doc = " this must, at minimum, return the starting position of the next match"] # [doc = " in the given haystack after or at the given position."] fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate ; }