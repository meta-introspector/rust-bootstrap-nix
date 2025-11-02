use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " A type that wraps a packed searcher and implements the `Prefilter`"] # [doc = " interface."] struct Packed (packed :: Searcher) ;