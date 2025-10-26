use std::collections::HashMap;
pub struct MyStruct { pub field: i32, }

impl MyStruct {
    pub fn new(field: i32) -> Self {
        Self { field }
    }
}

pub fn my_function() { println!("Hello!"); }

pub enum MyEnum { Variant1, Variant2, }
