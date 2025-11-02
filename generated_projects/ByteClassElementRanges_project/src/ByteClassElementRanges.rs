use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " An iterator over all elements in an equivalence class expressed as a"] # [doc = " sequence of contiguous ranges."] pub (crate) struct ByteClassElementRanges < 'a > { elements : ByteClassElements < 'a > , range : Option < (u8 , u8) > , }