use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct BagOfWordsVisitor { pub bag_of_words : HashMap < String , usize > , }