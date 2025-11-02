use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct Test { pub name : String , # [serde (flatten)] pub outcome : TestOutcome , }