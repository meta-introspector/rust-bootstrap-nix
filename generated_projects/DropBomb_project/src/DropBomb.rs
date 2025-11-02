use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct DropBomb { command : OsString , defused : bool , armed_location : panic :: Location < 'static > , }