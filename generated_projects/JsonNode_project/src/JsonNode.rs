use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [serde (tag = "kind" , rename_all = "snake_case")] pub enum JsonNode { RustbuildStep { # [serde (rename = "type")] type_ : String , debug_repr : String , duration_excluding_children_sec : f64 , system_stats : JsonStepSystemStats , children : Vec < JsonNode > , } , TestSuite (TestSuite) , }