#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*

"# [doc = \" Information about an import statement found in the AST\"] pub struct ImportInfo { pub path : String , pub alias : Option < String > , pub is_external : bool , pub source_crate : Option < String > , pub git_source_url : Option < String > , pub git_branch : Option < String > , }"