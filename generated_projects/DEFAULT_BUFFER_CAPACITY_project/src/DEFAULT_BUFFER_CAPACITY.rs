use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " The default buffer capacity that we use for the stream buffer."] const DEFAULT_BUFFER_CAPACITY : usize = 64 * (1 << 10) ;