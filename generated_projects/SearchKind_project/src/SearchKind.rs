use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

enum SearchKind { Teddy (teddy :: Searcher) , RabinKarp , }