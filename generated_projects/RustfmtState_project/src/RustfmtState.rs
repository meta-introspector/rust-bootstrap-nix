use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub enum RustfmtState { SystemToolchain (PathBuf) , Downloaded (PathBuf) , Unavailable , # [default] LazyEvaluated , }