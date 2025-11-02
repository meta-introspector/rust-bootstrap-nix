use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub trait CargoInfoTrait : Send + Sync + Debug { fn package_name (& self) -> Option < & str > ; fn version (& self) -> Option < & str > ; }