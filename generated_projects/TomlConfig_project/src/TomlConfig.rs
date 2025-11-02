use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [serde (deny_unknown_fields , rename_all = "kebab-case")] pub struct TomlConfig { # [serde (flatten)] change_id : ChangeIdWrapper , install : Option < Install > , pub llvm : Option < Llvm > , pub rust : Option < Rust > , target : Option < HashMap < String , TomlTarget > > , dist : Option < Dist > , ci : Option < Ci > , nix : Option < Nix > , profile : Option < String > , stage0_path : Option < PathBuf > , }