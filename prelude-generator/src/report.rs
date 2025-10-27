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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_generate_report_empty_results() -> Result<()> {
        let dir = tempdir()?;
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(&dir)?;

        generate_report(&[])?;

        let report_path = dir.path().join("prelude_generator_summary.md");
        assert!(report_path.exists());
        let content = fs::read_to_string(&report_path)?;
        assert!(content.contains("# Prelude Generation Summary Report"));
        assert!(content.contains("- Total files processed: 0"));
        assert!(!content.contains("## Detailed Results"));

        std::env::set_current_dir(&original_dir)?;
        Ok(())
    }

    #[test]
    fn test_generate_report_with_results() -> Result<()> {
        let dir = tempdir()?;
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(&dir)?;

        let results = vec![
            FileProcessingResult {
                path: PathBuf::from("src/file1.rs"),
                status: FileProcessingStatus::Success,
            },
            FileProcessingResult {
                path: PathBuf::from("src/file2.rs"),
                status: FileProcessingStatus::Skipped { reason: "already processed".to_string() },
            },
            FileProcessingResult {
                path: PathBuf::from("src/file3.rs"),
                status: FileProcessingStatus::Failed { error: "syntax error".to_string() },
            },
        ];

        generate_report(&results)?;

        let report_path = dir.path().join("prelude_generator_summary.md");
        assert!(report_path.exists());
        let content = fs::read_to_string(&report_path)?;

        assert!(content.contains("# Prelude Generation Summary Report"));
        assert!(content.contains("- Total files processed: 3"));
        assert!(content.contains("- Successfully processed: 1"));
        assert!(content.contains("- Skipped: 1"));
        assert!(content.contains("- Failed: 1"));
        assert!(content.contains("### src/file1.rs\n- Status: ✅ Successfully Processed"));
        assert!(content.contains("### src/file2.rs\n- Status: ⏭️ Skipped (Reason: already processed"));
        assert!(content.contains("### src/file3.rs\n- Status: ❌ Failed (Error: syntax error"));

        std::env::set_current_dir(&original_dir)?;
        Ok(())
    }
}