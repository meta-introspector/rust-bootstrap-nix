use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [serde (untagged)] pub enum StringOrBool { String (String) , Bool (bool) , }