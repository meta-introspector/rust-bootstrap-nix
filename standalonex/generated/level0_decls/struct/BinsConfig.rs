use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::string::String;
# [derive (Debug , Deserialize , Clone)] pub struct BinsConfig { # [serde (flatten)] pub paths : HashMap < String , PathBuf > , }