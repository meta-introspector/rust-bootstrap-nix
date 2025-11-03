#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

pub trait ConfigApplicator { fn apply_to_config (& self , config : & mut ParsedConfig , toml : & LocalTomlConfig) ; }