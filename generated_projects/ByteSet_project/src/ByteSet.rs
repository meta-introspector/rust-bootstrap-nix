use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " A simple set of bytes that is reasonably cheap to copy and allocation free."] pub (crate) struct ByteSet { bits : BitSet , }