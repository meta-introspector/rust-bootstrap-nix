use anyhow::{Context, Result};
use std::pin::Pin;
use std::future::Future;
use std::boxed::Box;
use tokio::io::AsyncWriteExt;

use crate::measurement;
use crate::code_generator;
use crate::prelude_category_pipeline::{PipelineFunctor, ParsedFile, UseStatements};

use syn;

// ExtractUsesFunctor
pub struct ExtractUsesFunctor;

impl PipelineFunctor<ParsedFile, UseStatements> for ExtractUsesFunctor {
    fn map<'writer>(
        &'writer self,
        _writer: &'writer mut (impl tokio::io::AsyncWriteExt + Unpin + Send),
        input: ParsedFile,
    ) -> Pin<Box<dyn Future<Output = Result<UseStatements>> + Send + 'writer>> {
        Box::pin(async move {
            measurement::record_function_entry("ExtractUsesFunctor::map");
            let ParsedFile(source_code, _) = input;

            let use_statements = tokio::task::spawn_blocking(move || -> anyhow::Result<_> {
                let ast = syn::parse_file(&source_code)
                    .context("Failed to parse source code for use statement extraction")?;
                let mut use_statements = Vec::new();
                for item in ast.items {
                    if let syn::Item::Use(use_item) = item {
                        use_statements.push(code_generator::use_item_to_string(&use_item));
                    }
                }
                Ok(UseStatements(use_statements))
            }).await.context("Blocking task for extracting use statements failed")??;

            measurement::record_function_exit("ExtractUsesFunctor::map");
            Ok(use_statements)
        })
    }
}
