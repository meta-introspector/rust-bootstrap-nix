use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub (super) struct SlimNeon < const BYTES : usize > { slim128 : generic :: Slim < uint8x16_t , BYTES > , }