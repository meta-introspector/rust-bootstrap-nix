use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::string::String;
# [derive (Debug , Serialize , Deserialize , PartialEq , Eq , Hash , Clone)] pub struct TestInfo { pub name : String , pub file_path : PathBuf , }