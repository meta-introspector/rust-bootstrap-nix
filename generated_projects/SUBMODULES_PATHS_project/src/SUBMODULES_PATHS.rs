use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

static SUBMODULES_PATHS : OnceLock < Vec < String > > = OnceLock :: new () ;