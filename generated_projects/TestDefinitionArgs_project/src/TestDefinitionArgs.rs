use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

struct TestDefinitionArgs { name : Ident , path : LitStr , mode : LitStr , suite : LitStr , default : LitBool , host : LitBool , compare_mode : syn :: Expr , }