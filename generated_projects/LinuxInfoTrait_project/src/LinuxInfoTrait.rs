use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub trait LinuxInfoTrait : Send + Sync + Debug { fn kernel_version (& self) -> Option < & str > ; fn architecture (& self) -> Option < & str > ; }