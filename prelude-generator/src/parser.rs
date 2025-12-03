use anyhow::{Context, Result};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use crate::measurement;
use prettyplease;
use syn; // Add this import // Add this import

use pipeline_traits::Config as PipelineConfig;
use pipeline_traits::{ParsedFile, PipelineFunctor, RawFile};
// ParseFunctor
#[allow(dead_code)] // Suppress dead_code warning for ParseFunctor
pub struct ParseFunctor;
impl PipelineFunctor<RawFile, ParsedFile, PipelineConfig> for ParseFunctor {
    fn map<'writer>(
        &'writer self,
        _writer: &'writer mut (impl tokio::io::AsyncWriteExt + Unpin + Send),
        input: RawFile,
        _config: &'writer Option<PipelineConfig>,
    ) -> Pin<Box<dyn Future<Output = Result<ParsedFile>> + Send + 'writer>> {
        Box::pin(async move {
            measurement::record_function_entry("ParseFunctor::map");
            let RawFile(file_path_str, content) = input;
            let file_path = PathBuf::from(file_path_str.clone());

            let parsed_code = tokio::task::spawn_blocking(move || -> anyhow::Result<_> {
                let ast = match syn::parse_file(&content) {
                    Ok(ast) => ast,
                    Err(_) => {
                        return Err(anyhow::anyhow!("Failed to parse file and expand macros"));
                    }
                };
                Ok(prettyplease::unparse(&ast))
            })
            .await
            .context("Blocking task for parsing failed")??;

            measurement::record_function_exit("ParseFunctor::map");
            Ok(ParsedFile(parsed_code, file_path))
        })
    }
}
