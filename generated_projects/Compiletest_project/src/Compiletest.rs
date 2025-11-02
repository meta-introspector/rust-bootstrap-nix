use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct Compiletest { pub compiler : Compiler , pub target : TargetSelection , pub mode : & 'static str , pub suite : & 'static str , pub path : & 'static str , pub compare_mode : Option < & 'static str > , }