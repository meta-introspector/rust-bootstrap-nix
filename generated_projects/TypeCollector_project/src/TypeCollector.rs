use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct TypeCollector < 'a > { pub type_map : & 'a mut HashMap < String , TypeInfo > , }