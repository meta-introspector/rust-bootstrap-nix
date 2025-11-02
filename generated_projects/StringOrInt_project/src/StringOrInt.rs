use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [serde (untagged)] pub enum StringOrInt { String (String) , Int (i64) , }