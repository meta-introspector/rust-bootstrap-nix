use std::collections::HashMap;
use crate::type_extractor::TypeInfo;
pub struct TypeCollector < 'a > { pub type_map : & 'a mut HashMap < String , TypeInfo > , }