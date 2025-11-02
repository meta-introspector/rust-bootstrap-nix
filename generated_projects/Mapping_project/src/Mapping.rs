use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

struct Mapping { # [serde (flatten)] files : HashMap < String , String > , }