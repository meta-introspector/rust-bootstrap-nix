#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate std;
#[prelude_import]
use ::std::prelude::rust_2015::*;
pub mod prelude {


    // This will be the lib.rs for the new bootstrap-test-utils crate
    pub use std::path::Path;
}
