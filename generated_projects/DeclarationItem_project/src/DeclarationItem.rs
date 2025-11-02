use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub enum DeclarationItem { Const (ItemConst) , Struct (ItemStruct) , Enum (ItemEnum) , Fn (ItemFn) , Static (ItemStatic) , Other (syn :: Item) , }