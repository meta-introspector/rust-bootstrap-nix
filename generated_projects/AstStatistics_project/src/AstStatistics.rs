use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " Comprehensive AST analysis data for a Rust project"] pub struct AstStatistics { pub node_type_counts : HashMap < String , u32 > , pub variable_declarations : Vec < VariableInfo > , pub function_definitions : Vec < FunctionInfo > , pub import_statements : Vec < ImportInfo > , }