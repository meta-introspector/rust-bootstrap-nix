use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub enum LldMode { # [doc = " Do not use LLD"] # [default] Unused , # [doc = " Use `rust-lld` from the compiler's sysroot"] SelfContained , # [doc = " Use an externally provided `lld` binary."] # [doc = " Note that the linker name cannot be overridden, the binary has to be named `lld` and it has"] # [doc = " to be in $PATH."] External , }