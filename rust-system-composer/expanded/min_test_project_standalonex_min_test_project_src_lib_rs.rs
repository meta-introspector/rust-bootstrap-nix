#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate std;
#[prelude_import]
use ::std::prelude::rust_2015::*;
pub const MIN_TEST_NUM: u32 = 42;
pub struct MinTestStruct {
    pub id: u32,
}
