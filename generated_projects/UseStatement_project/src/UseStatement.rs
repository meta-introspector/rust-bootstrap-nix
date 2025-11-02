use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct UseStatement { pub statement : String , pub error : Option < String > , pub git_details : Option < GitDetails > , pub nix_details : Option < NixDetails > , pub rust_details : Option < RustDetails > , pub cargo_details : Option < CargoDetails > , pub syn_details : Option < SynDetails > , pub llvm_details : Option < LlvmDetails > , pub linux_details : Option < LinuxDetails > , }