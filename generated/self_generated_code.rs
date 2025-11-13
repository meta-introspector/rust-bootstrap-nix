use anyhow::Result;
use std::pin::Pin;
use std::future::Future;
use std::boxed::Box;
use crate::measurement;
use pipeline_traits::{PipelineFunctor, ClassifiedUseStatements, UseStatement};
pub struct PreprocessFunctor;
impl PipelineFunctor<ClassifiedUseStatements, ClassifiedUseStatements>
for PreprocessFunctor {
    fn map<'writer>(
        &'writer self,
        _writer: &'writer mut (impl tokio::io::AsyncWriteExt + Unpin + Send),
        input: ClassifiedUseStatements,
    ) -> Pin<
        Box<dyn Future<Output = Result<ClassifiedUseStatements>> + Send + 'writer>,
    > {
        Box::pin(async move {
            measurement::record_function_entry("PreprocessFunctor::map");
            let ClassifiedUseStatements(classified_uses) = input;
            let mut new_classified_uses = Vec::new();
            for use_statement in classified_uses {
                if use_statement.error.is_some() {
                    let temp_dir = tempfile::tempdir()?;
                    let temp_file_path = temp_dir.path().join("main.rs");
                    let content = format!("{}\nfn main() {{}}", use_statement.statement);
                    tokio::fs::write(&temp_file_path, content).await?;
                    let output = tokio::process::Command::new("rustc")
                        .arg(&temp_file_path)
                        .output()
                        .await?;
                    if output.status.success() {
                        new_classified_uses
                            .push(UseStatement {
                                statement: use_statement.statement,
                                error: None,
                                git_details: None,
                                nix_details: None,
                                rust_details: None,
                                cargo_details: None,
                                syn_details: None,
                                llvm_details: None,
                                linux_details: None,
                            });
                    } else {
                        new_classified_uses
                            .push(UseStatement {
                                statement: use_statement.statement,
                                error: Some(
                                    String::from_utf8_lossy(&output.stderr).to_string(),
                                ),
                                git_details: None,
                                nix_details: None,
                                rust_details: None,
                                cargo_details: None,
                                syn_details: None,
                                llvm_details: None,
                                linux_details: None,
                            });
                    }
                } else {
                    new_classified_uses
                        .push(UseStatement {
                            statement: use_statement.statement,
                            error: use_statement.error,
                            git_details: use_statement.git_details,
                            nix_details: use_statement.nix_details,
                            rust_details: use_statement.rust_details,
                            cargo_details: use_statement.cargo_details,
                            syn_details: use_statement.syn_details,
                            llvm_details: use_statement.llvm_details,
                            linux_details: use_statement.linux_details,
                        });
                }
            }
            let __result = Ok(ClassifiedUseStatements(new_classified_uses));
            measurement::record_function_exit("PreprocessFunctor::map");
            __result
        })
    }
}
