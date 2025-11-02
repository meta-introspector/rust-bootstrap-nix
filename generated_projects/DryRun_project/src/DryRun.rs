use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub enum DryRun { # [doc = " This isn't a dry run."] # [default] Disabled , # [doc = " This is a dry run enabled by bootstrap itself, so it can verify that no work is done."] SelfCheck , # [doc = " This is a dry run enabled by the `--dry-run` flag."] UserSelected , }