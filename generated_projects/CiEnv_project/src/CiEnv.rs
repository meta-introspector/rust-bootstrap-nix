use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub enum CiEnv { # [doc = " Not a CI environment."] None , # [doc = " The GitHub Actions environment, for Linux (including Docker), Windows and macOS builds."] GitHubActions , }