use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " A single chunk yielded by the stream chunk iterator."] # [doc = ""] # [doc = " The `'r` lifetime refers to the lifetime of the stream chunk iterator."] enum StreamChunk < 'r > { # [doc = " A chunk that does not contain any matches."] NonMatch { bytes : & 'r [u8] } , # [doc = " A chunk that precisely contains a match."] Match { bytes : & 'r [u8] , mat : Match } , }