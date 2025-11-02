use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " Helper struct to traverse the AST and collect statistics"] struct AstVisitor { stats : AstStatistics , # [allow (dead_code)] file_path : PathBuf , }