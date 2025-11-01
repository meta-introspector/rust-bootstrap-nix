use anyhow::Result;
use std::fs;
use prelude_collector::{FileProcessingResult, FileProcessingStatus};

/// Generates a markdown report of the file processing results.
pub fn generate_report(results: &[FileProcessingResult]) -> Result<()> {
    let mut report_content = String::new();
    report_content.push_str("# Prelude Generation Summary Report\n\n");
    report_content.push_str("This report summarizes the processing of Rust files during prelude generation.\n\n");

    let total_files = results.len();
    let successful_files = results.iter().filter(|r| matches!(r.status, FileProcessingStatus::Success)).count();
    let skipped_files = results.iter().filter(|r| matches!(r.status, FileProcessingStatus::Skipped { .. })).count();
    let failed_files = results.iter().filter(|r| matches!(r.status, FileProcessingStatus::Failed { .. })).count();

    report_content.push_str("## Summary\n");
    report_content.push_str(&format!("- Total files processed: {}\n", total_files));
    report_content.push_str(&format!("- Successfully processed: {}\n", successful_files));
    report_content.push_str(&format!("- Skipped: {}\n", skipped_files));
    report_content.push_str(&format!("- Failed: {}\n\n", failed_files));

    if !results.is_empty() {
        report_content.push_str("## Detailed Results\n");
        for result in results {
            report_content.push_str(&format!("### {}\n", result.path.display()));
            match &result.status {
                FileProcessingStatus::Success => {
                    report_content.push_str("- Status: ✅ Successfully Processed\n\n");
                }
                FileProcessingStatus::Skipped { reason } => {
                    report_content.push_str(&format!("- Status: ⏭️ Skipped (Reason: {}\n\n", reason));
                }
                FileProcessingStatus::Failed { error } => {
                    report_content.push_str(&format!("- Status: ❌ Failed (Error: {}\n\n", error));
                }
            }
        }
    }

    fs::write("prelude_generator_summary.md", report_content)?;
    println!("Report generated: prelude_generator_summary.md");
    Ok(())
}