use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub fn extract_rustc_version_from_path (path : & str) -> String { let parts : Vec < & str > = path . split ('/') . collect () ; if parts . len () >= 4 { let store_dir = parts [3] ; if let Some (dash_index) = store_dir . find ('-') { if let Some (second_dash_index) = store_dir [dash_index + 1 ..] . find ('-') { return store_dir [dash_index + 1 + second_dash_index ..] . to_string () ; } } store_dir . to_string () } else { "unknown" . to_string () } }