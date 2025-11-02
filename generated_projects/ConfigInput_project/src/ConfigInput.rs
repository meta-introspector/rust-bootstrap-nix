use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

struct ConfigInput { attrs : Vec < syn :: Attribute > , ident : Ident , fields : syn :: punctuated :: Punctuated < ConfigField , :: syn :: token :: Comma > , }