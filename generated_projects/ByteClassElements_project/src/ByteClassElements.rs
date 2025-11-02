use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " An iterator over all elements in a specific equivalence class."] pub (crate) struct ByteClassElements < 'a > { classes : & 'a ByteClasses , class : u8 , bytes : core :: ops :: RangeInclusive < u8 > , }