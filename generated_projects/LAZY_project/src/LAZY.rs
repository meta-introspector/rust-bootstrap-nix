use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

static LAZY : :: lazy_static :: lazy :: Lazy < Mutex < HashMap < String , FunctionMetrics > > , > = :: lazy_static :: lazy :: Lazy :: INIT ;