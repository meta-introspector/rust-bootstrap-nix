use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct GitConfig < 'a > { pub git_repository : & 'a str , pub nightly_branch : & 'a str , pub git_merge_commit_email : & 'a str , }