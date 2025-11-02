use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub fn threads_from_config (v : u32) -> u32 { match v { 0 => { std :: thread :: available_parallelism () . map_or (1 , std :: num :: NonZeroUsize :: get) as u32 } n => n , } }