pub mod args;
pub mod report;
pub mod generate_prelude;
pub mod modify_file;
pub mod modify_crate_root;
pub mod processor;
pub mod test_extractor;
pub mod pipeline;
pub mod use_extractor;
//#[path = "prelude_category_pipeline.rs"]
pub mod prelude_category_pipeline;
pub mod code_generator;
pub mod measurement;
pub mod hf_dataset_reader;
//#[path = "parser.rs"]
pub mod parser; // Add this line
pub use args::Args;
pub use report::generate_report;
pub use generate_prelude::generate_prelude;
pub use modify_file::modify_file;
pub use modify_crate_root::modify_crate_root;
pub use processor::process_crates;
pub use test_extractor::{collect_all_test_cases, generate_test_report_json, generate_test_verification_script_and_report, TestInfo};

// Re-export necessary types from prelude_collector
pub use prelude_collector::{FileProcessingResult, FileProcessingStatus};
