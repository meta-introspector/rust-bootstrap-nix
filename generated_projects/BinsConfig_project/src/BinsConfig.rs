use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct BinsConfig { # [serde (flatten)] pub paths : HashMap < String , PathBuf > , }