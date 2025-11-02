use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [command (author , version , about , long_about = None)] struct Args { # [doc = " Path to the project root to search for Cargo.toml files."] # [arg (long , default_value = ".")] project_root : PathBuf , # [doc = " Path to the rustc wrapper script."] # [arg (long)] rustc_wrapper_path : PathBuf , # [doc = " Output directory for generated flakes."] # [arg (long , default_value = "generated_flakes")] output_dir : PathBuf , }