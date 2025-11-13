/// Documentation generation for bootstrap.
///
/// This module implements generation for all bits and pieces of documentation
/// for the Rust project. This notably includes suites like the rust book, the
/// nomicon, rust by example, standalone documentation, etc.
///
/// Everything here is basically just a shim around calling either `rustbook` or
/// `rustdoc`.
use std::io::{self, Write};
