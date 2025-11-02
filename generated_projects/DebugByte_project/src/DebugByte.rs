use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " A type that wraps a single byte with a convenient fmt::Debug impl that"] # [doc = " escapes the byte."] pub (crate) struct DebugByte (pub (crate) u8) ;