use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " A single transition in a non-contiguous NFA."] # [repr (packed)] pub (crate) struct Transition { byte : u8 , next : StateID , link : StateID , }