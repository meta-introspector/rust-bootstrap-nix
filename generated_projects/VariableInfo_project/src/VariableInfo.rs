use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " Information about a variable found in the AST"] pub struct VariableInfo { pub name : String , pub type_name : String , pub is_mutable : bool , pub scope : String , }