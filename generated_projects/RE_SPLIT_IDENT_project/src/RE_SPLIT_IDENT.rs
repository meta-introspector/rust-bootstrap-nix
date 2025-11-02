use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

static RE_SPLIT_IDENT : Lazy < Regex > = Lazy :: new (| | { Regex :: new (r"[^a-zA-Z0-9]+|(?<=[a-z])(?=[A-Z])|^_|_$") . unwrap () }) ;