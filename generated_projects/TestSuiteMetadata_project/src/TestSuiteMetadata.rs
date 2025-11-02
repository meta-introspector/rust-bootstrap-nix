use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [serde (tag = "kind" , rename_all = "snake_case")] pub enum TestSuiteMetadata { CargoPackage { crates : Vec < String > , target : String , host : String , stage : u32 } , Compiletest { suite : String , mode : String , compare_mode : Option < String > , target : String , host : String , stage : u32 , } , }