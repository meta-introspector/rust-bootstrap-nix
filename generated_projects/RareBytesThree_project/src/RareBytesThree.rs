use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " A prefilter for scanning for three \"rare\" bytes."] struct RareBytesThree { offsets : RareByteOffsets , byte1 : u8 , byte2 : u8 , byte3 : u8 , }