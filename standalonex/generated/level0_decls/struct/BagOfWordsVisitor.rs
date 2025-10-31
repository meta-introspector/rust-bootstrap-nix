use std::collections::HashMap;
use std::string::String;
use syn::visit::Visit;
pub struct BagOfWordsVisitor { pub bag_of_words : HashMap < String , usize > , }