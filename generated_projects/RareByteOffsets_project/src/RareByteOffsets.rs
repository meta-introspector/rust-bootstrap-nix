use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " A set of byte offsets, keyed by byte."] struct RareByteOffsets { # [doc = " Each entry corresponds to the maximum offset of the corresponding"] # [doc = " byte across all patterns seen."] set : [RareByteOffset ; 256] , }