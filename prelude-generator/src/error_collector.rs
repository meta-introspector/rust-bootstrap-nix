use serde::{Serialize, Deserialize};
use std::path::{PathBuf, Path};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorSample {
    pub file_path: PathBuf,
    pub rustc_version: String,
    pub rustc_host: String,
    pub error_message: String,
    pub error_type: String, // e.g., "MacroExpansionFailed", "SynParsingFailed"
    pub code_snippet: Option<String>, // The code that caused the error
    pub timestamp: DateTime<Utc>,
    pub context: Option<String>, // Additional context if available
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ErrorCollection {
    pub errors: Vec<ErrorSample>,
}

impl ErrorCollection {
    pub fn add_error(&mut self, error_sample: ErrorSample) {
        self.errors.push(error_sample);
    }

    pub async fn write_to_file(&self, output_path: &Path) -> anyhow::Result<()> {
        let json_content = serde_json::to_string_pretty(&self.errors)?;
        tokio::fs::write(output_path, json_content).await?;
        Ok(())
    }
}
