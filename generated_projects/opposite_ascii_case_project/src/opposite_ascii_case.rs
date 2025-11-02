use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " If the given byte is an ASCII letter, then return it in the opposite case."] # [doc = " e.g., Given `b'A'`, this returns `b'a'`, and given `b'a'`, this returns"] # [doc = " `b'A'`. If a non-ASCII letter is given, then the given byte is returned."] pub (crate) fn opposite_ascii_case (b : u8) -> u8 { if b'A' <= b && b <= b'Z' { b . to_ascii_lowercase () } else if b'a' <= b && b <= b'z' { b . to_ascii_uppercase () } else { b } }