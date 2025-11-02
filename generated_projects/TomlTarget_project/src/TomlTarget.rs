use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct TomlTarget { pub llvm_libunwind : Option < String > , pub split_debuginfo : Option < String > , pub profiler : Option < StringOrBool > , pub rpath : Option < bool > , pub llvm : Option < bool > , }