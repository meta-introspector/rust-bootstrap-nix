use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct DistConfig { # [serde (rename = "sign-folder")] pub sign_folder : Option < String > , # [serde (rename = "upload-addr")] pub upload_addr : Option < String > , }