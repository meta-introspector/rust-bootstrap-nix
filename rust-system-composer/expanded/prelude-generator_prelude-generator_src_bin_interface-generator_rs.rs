#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate std;
#[prelude_import]
use ::std::prelude::rust_2015::*;
// This file is a placeholder for the interface-generator binary.
// Its actual content will be added later.
fn main() {
    { ::std::io::_print(format_args!("interface-generator placeholder\n")); };
}
