#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate std;
#[prelude_import]
use ::std::prelude::rust_2015::*;
pub mod prelude {




    pub use super::*;
}
pub fn add(left: u64, right: u64) -> u64 { left + right }
