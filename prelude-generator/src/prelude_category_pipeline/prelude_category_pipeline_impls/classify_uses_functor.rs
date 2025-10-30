use anyhow::{Result};
use std::pin::Pin;
use std::future::Future;
use std::boxed::Box;

use crate::measurement;
use pipeline_traits::{PipelineFunctor, UseStatements, ClassifiedUseStatements, UseStatement};
use syn;

// ClassifyUsesFunctor
pub struct ClassifyUsesFunctor;

impl PipelineFunctor<UseStatements, ClassifiedUseStatements> for ClassifyUsesFunctor {
    fn map<'writer>(
        &'writer self,
        _writer: &'writer mut (impl tokio::io::AsyncWriteExt + Unpin + Send),
        input: UseStatements,
    ) -> Pin<Box<dyn Future<Output = Result<ClassifiedUseStatements>> + Send + 'writer>> {
        Box::pin(async move {
            measurement::record_function_entry("ClassifyUsesFunctor::map");
            let UseStatements(use_statements) = input;
            let mut classified_uses = Vec::new();
            for use_statement in use_statements {
                match syn::parse_str::<syn::ItemUse>(&use_statement) {
                    Ok(_) => classified_uses.push(UseStatement {
                        statement: use_statement,
                        error: None,
                        git_details: None,
                        nix_details: None,
                        rust_details: None,
                        cargo_details: None,
                        syn_details: None,
                        llvm_details: None,
                        linux_details: None,
                    }),
                    Err(e) => classified_uses.push(UseStatement {
                        statement: use_statement,
                        error: Some(e.to_string()),
                        git_details: None,
                        nix_details: None,
                        rust_details: None,
                        cargo_details: None,
                        syn_details: None,
                        llvm_details: None,
                        linux_details: None,
                    }),
                }
            }
            let __result = Ok(ClassifiedUseStatements(classified_uses));
            measurement::record_function_exit("ClassifyUsesFunctor::map");
            __result
        })
    }
}
