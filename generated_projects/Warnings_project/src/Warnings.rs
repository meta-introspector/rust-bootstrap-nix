use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " Whether to deny warnings, emit them as warnings, or use the default behavior"] pub enum Warnings { Deny , Warn , # [default] Default , }