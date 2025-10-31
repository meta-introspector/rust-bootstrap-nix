use serde::{Serialize, Deserialize};
# [derive (Debug , Serialize , Deserialize)] pub struct UseStatement { pub statement : String , pub error : Option < String > , }