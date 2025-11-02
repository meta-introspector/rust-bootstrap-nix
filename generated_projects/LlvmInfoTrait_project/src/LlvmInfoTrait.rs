use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub trait LlvmInfoTrait : Send + Sync + Debug { fn ir_version (& self) -> Option < & str > ; fn target_triple (& self) -> Option < & str > ; }