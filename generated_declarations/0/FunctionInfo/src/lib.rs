#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*

"# [doc = \" Information about a function found in the AST\"] pub struct FunctionInfo { pub name : String , pub visibility : String , pub arg_count : u32 , pub arg_types : Vec < String > , pub return_type : String , pub is_async : bool , pub is_unsafe : bool , pub is_const : bool , }"