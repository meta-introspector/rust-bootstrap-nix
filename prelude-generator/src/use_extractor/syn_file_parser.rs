use anyhow::Result;
use std::path::Path;
use tokio::io::AsyncWriteExt;
use syn::File;
use crate::use_extractor::rustc_info::RustcInfo;
use split_expanded_lib::ErrorSample;

pub struct SynFileParser;

impl SynFileParser {
    pub async fn parse_expanded_code(
        _writer: &mut (impl tokio::io::AsyncWriteExt + Unpin),
        _file_path: &Path,
        expanded_code: String,
        _rustc_info: &RustcInfo,
    ) -> Result<(File, Option<ErrorSample>)> {
        Ok((syn::parse_file(&expanded_code)?, None))
    }
}