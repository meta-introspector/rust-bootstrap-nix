use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " LTO mode used for compiling rustc itself."] pub enum RustcLto { Off , # [default] ThinLocal , Thin , Fat , }