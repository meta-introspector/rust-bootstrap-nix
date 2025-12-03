pub mod collect_and_process_use_statements;
pub mod expand_macros_and_parse;
pub mod flatten_use_tree;
pub mod generate_aggregated_use_test_file;
pub mod rustc_info;
pub mod cache_manager; // Added
pub mod rustc_macro_expander; // Added
pub mod syn_file_parser; // Added placeholder
pub mod temp_crate_builder; // Added placeholder

pub use collect_and_process_use_statements::collect_and_process_use_statements;
pub use expand_macros_and_parse::expand_macros_and_parse;
pub use flatten_use_tree::flatten_use_tree;
pub use generate_aggregated_use_test_file::generate_aggregated_use_test_file;
pub use rustc_info::get_rustc_info;
pub use rustc_info::RustcInfo;
