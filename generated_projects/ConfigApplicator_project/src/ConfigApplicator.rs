use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub trait ConfigApplicator { fn apply_to_config (& self , config : & mut ParsedConfig , toml : & LocalTomlConfig) ; }