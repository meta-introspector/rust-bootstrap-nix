use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

fn output (cmd : & mut Command) -> Vec < u8 > { cmd . output () . expect ("command failed to run") . stdout }