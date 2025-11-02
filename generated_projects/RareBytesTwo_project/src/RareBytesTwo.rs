use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " A prefilter for scanning for two \"rare\" bytes."] struct RareBytesTwo { offsets : RareByteOffsets , byte1 : u8 , byte2 : u8 , }