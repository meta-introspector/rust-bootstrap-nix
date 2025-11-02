use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

fn is_in_gha () -> bool { std :: env :: var_os ("GITHUB_ACTIONS") . is_some () }