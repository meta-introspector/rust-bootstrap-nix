use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub fn format_new (template : & str , arg1 : & str , arg2 : & str) -> String { template . replacen ("{}" , arg1 , 1) . replace ("{}" , arg2) }