use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

struct Metadata { packages : Vec < Package > , # [allow (dead_code)] workspace_root : PathBuf , }