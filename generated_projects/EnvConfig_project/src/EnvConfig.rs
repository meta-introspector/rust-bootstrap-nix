use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct EnvConfig { # [serde (rename = "HOME")] pub home : Option < String > , # [serde (rename = "CARGO_HOME")] pub cargo_home : Option < String > , }