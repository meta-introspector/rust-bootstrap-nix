/// Implementation of the various distribution aspects of the compiler.
///
/// This module is responsible for creating tarballs of the standard library,
/// compiler, and documentation. This ends up being what we distribute to
/// everyone as well.
///
/// No tarball is actually created literally in this file, but rather we shell
/// out to `rust-installer` still. This may one day be replaced with bits and
/// pieces of `rustup.rs`!
use std::collections::HashSet;
