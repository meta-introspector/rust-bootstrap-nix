pub use my_struct::MyStruct;

impl MyStruct { pub fn new (field : i32) -> Self { MyStruct { field } } pub fn get_field (& self) -> i32 { self . field } }
