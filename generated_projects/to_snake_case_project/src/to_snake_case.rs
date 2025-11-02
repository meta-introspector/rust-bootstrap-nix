use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

fn to_snake_case (ident : & Ident) -> String { let mut s = String :: new () ; for (i , char) in ident . to_string () . chars () . enumerate () { if char . is_uppercase () && i != 0 { s . push ('_') ; } s . push (char . to_ascii_lowercase ()) ; } s }