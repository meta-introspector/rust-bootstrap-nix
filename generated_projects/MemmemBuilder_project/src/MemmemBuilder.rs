use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " A builder for constructing a prefilter that uses memmem."] struct MemmemBuilder { # [doc = " The number of patterns that have been added."] count : usize , # [doc = " The singular pattern to search for. This is only set when count==1."] one : Option < Vec < u8 > > , }