use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " The representation of a byte set. Split out so that we can define a"] # [doc = " convenient Debug impl for it while keeping \"ByteSet\" in the output."] struct BitSet ([u128 ; 2]) ;