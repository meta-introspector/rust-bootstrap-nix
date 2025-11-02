use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub trait SynInfoTrait : Send + Sync + Debug { fn parsed_type (& self) -> Option < & str > ; fn version (& self) -> Option < & str > ; }