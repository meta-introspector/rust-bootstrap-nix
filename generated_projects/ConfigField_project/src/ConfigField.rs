use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

struct ConfigField { ident : Ident , ty : syn :: Type , key : Option < LitStr > , }