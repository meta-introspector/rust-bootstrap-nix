use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " A prefilter for scanning for a single \"rare\" byte."] struct RareBytesOne { byte1 : u8 , offset : RareByteOffset , }