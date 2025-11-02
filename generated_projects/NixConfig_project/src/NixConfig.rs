use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct NixConfig { pub nixpkgs_path : Option < String > , pub rust_overlay_path : Option < String > , pub rust_bootstrap_nix_path : Option < String > , pub configuration_nix_path : Option < String > , pub rust_src_flake_path : Option < PathBuf > , pub rust_bootstrap_nix_flake_ref : Option < String > , pub rust_src_flake_ref : Option < String > , }