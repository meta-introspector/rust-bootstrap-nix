use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub trait RustDetailsInfoTrait : Send + Sync + Debug { fn version (& self) -> Option < & str > ; fn crate_name (& self) -> Option < & str > ; fn item_path (& self) -> Option < & str > ; }