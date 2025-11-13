pub fn my_function_one() -> i32 {
    1 + 2
}

pub fn my_function_two() -> usize {
    "hello".len()
}

pub struct MyStruct {
    field: i32,
}

impl MyStruct {
    pub fn new(field: i32) -> Self {
        MyStruct { field }
    }

    pub fn get_field(&self) -> i32 {
        self.field
    }
}
