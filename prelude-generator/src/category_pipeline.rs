use anyhow::{Context, Result};
use crate::pipeline::UseStatement;
use crate::use_extractor;
use std::path::{Path, PathBuf};
use crate::code_generator;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use std::fmt::Debug;
use crate::measurement;


use tempfile::{tempdir, NamedTempFile};
use tokio::process;

// InspectFunctor
pub struct InspectFunctor<'a, T: Debug> {
    label: &'a str,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: Debug> InspectFunctor<'a, T> {
    pub fn new(label: &'a str) -> Self {
        InspectFunctor {
            label,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, T: Debug + Clone + Send> PipelineFunctor<T, T> for InspectFunctor<'a, T> {
    fn map(&self, writer: &mut (impl tokio::io::AsyncWriteExt + Unpin + Send), input: T) -> impl std::future::Future<Output = Result<T>> {
        async move {
            writer.write_all(format!("--- Inspecting: {} ---\n", self.label).as_bytes()).await?;
            writer.write_all(format!("{:#?}\n", input).as_bytes()).await?;
            Ok(input)
        }
    }
}
#[derive(Debug)]
pub struct RawFile(pub String, pub String);
#[derive(Clone)]
pub struct ParsedFile(pub syn::File);
#[derive(Debug)]
pub struct UseStatements(pub Vec<String>);
#[derive(Debug)]
pub struct ClassifiedUseStatements(pub Vec<UseStatement>);

// Functors (as a trait)
pub trait PipelineFunctor<Input, Output> {
    fn map(&self, writer: &mut (impl tokio::io::AsyncWriteExt + Unpin + Send), input: Input) -> impl std::future::Future<Output = Result<Output>>;
}

// ParseFunctor
pub struct ParseFunctor;

impl PipelineFunctor<RawFile, ParsedFile> for ParseFunctor {
    fn map(&self, writer: &mut (impl tokio::io::AsyncWriteExt + Unpin + Send), input: RawFile) -> impl std::future::Future<Output = Result<ParsedFile>> {
        async move {
            measurement::record_function_entry("ParseFunctor::map");
            let RawFile(file_path, content) = input;
            let __result = match syn::parse_file(&content) {
                Ok(ast) => Ok(ParsedFile(ast)),
                Err(_) => {
                    let rustc_info = use_extractor::get_rustc_info()?;
                    let cache_dir = Path::new(".").join(".prelude_cache");
                    let ast = use_extractor::expand_macros_and_parse(
                        writer, // Pass the writer here
                        Path::new(&file_path),
                        &content,
                        &rustc_info,
                        &cache_dir,
                    ).await?;
                    Ok(ParsedFile(ast))
                }
            };
            measurement::record_function_exit("ParseFunctor::map");
            __result
        }
    }
}

// ExtractUsesFunctor
pub struct ExtractUsesFunctor;

impl PipelineFunctor<ParsedFile, UseStatements> for ExtractUsesFunctor {
    fn map(&self, _writer: &mut (impl tokio::io::AsyncWriteExt + Unpin + Send), input: ParsedFile) -> impl std::future::Future<Output = Result<UseStatements>> {
        async move {
            measurement::record_function_entry("ExtractUsesFunctor::map");
            let ParsedFile(ast) = input;
            let mut use_statements = Vec::new();
            for item in ast.items {
                if let syn::Item::Use(use_item) = item {
                    use_statements.push(code_generator::use_item_to_string(&use_item));
                }
            }
            let __result = Ok(UseStatements(use_statements));
            measurement::record_function_exit("ExtractUsesFunctor::map");
            __result
        }
    }
}

// ClassifyUsesFunctor
pub struct ClassifyUsesFunctor;

impl PipelineFunctor<UseStatements, ClassifiedUseStatements> for ClassifyUsesFunctor {
    fn map(&self, _writer: &mut (impl tokio::io::AsyncWriteExt + Unpin + Send), input: UseStatements) -> impl std::future::Future<Output = Result<ClassifiedUseStatements>> {
        async move {
            measurement::record_function_entry("ClassifyUsesFunctor::map");
            let UseStatements(use_statements) = input;
            let mut classified_uses = Vec::new();
            for use_statement in use_statements {
                match syn::parse_str::<syn::ItemUse>(&use_statement) {
                    Ok(_) => classified_uses.push(UseStatement {
                        statement: use_statement,
                        error: None,
                    }),
                    Err(e) => classified_uses.push(UseStatement {
                        statement: use_statement,
                        error: Some(e.to_string()),
                    }),
                }
            }
            let __result = Ok(ClassifiedUseStatements(classified_uses));
            measurement::record_function_exit("ClassifyUsesFunctor::map");
            __result
        }
    }
}

// PreprocessFunctor
pub struct PreprocessFunctor;

impl PipelineFunctor<ClassifiedUseStatements, ClassifiedUseStatements> for PreprocessFunctor {
    fn map(&self, _writer: &mut (impl tokio::io::AsyncWriteExt + Unpin + Send), input: ClassifiedUseStatements) -> impl std::future::Future<Output = Result<ClassifiedUseStatements>> {
        async move {
            measurement::record_function_entry("PreprocessFunctor::map");
            let ClassifiedUseStatements(classified_uses) = input;
            let mut new_classified_uses = Vec::new();
            for use_statement in classified_uses {
                if use_statement.error.is_some() {
                    let temp_dir = tempfile::tempdir()?;
                    let temp_file_path = temp_dir.path().join("main.rs");
                    let content = format!("{}\nfn main() {{}}", use_statement.statement);
                    tokio::fs::write(&temp_file_path, content).await?;

                    let output = tokio::process::Command::new("rustc") // Use tokio::process::Command
                        .arg(&temp_file_path)
                        .output().await?; // Add await

                    if output.status.success() {
                        new_classified_uses.push(UseStatement {
                            statement: use_statement.statement,
                            error: None,
                        });
                    } else {
                        new_classified_uses.push(UseStatement {
                            statement: use_statement.statement,
                            error: Some(String::from_utf8_lossy(&output.stderr).to_string()),
                        });
                    }
                } else {
                    new_classified_uses.push(use_statement);
                }
            }
            let __result = Ok(ClassifiedUseStatements(new_classified_uses));
            measurement::record_function_exit("PreprocessFunctor::map");
            __result
        }
    }
}

#[derive(Debug)]
pub struct ValidatedFile(pub PathBuf); // Now stores the path to the generated dataset

// HuggingFaceValidatorFunctor
pub struct HuggingFaceValidatorFunctor;

impl PipelineFunctor<ParsedFile, ValidatedFile> for HuggingFaceValidatorFunctor {
    fn map(&self, writer: &mut (impl tokio::io::AsyncWriteExt + Unpin + Send), input: ParsedFile) -> impl std::future::Future<Output = Result<ValidatedFile>> {
        async move {
            measurement::record_function_entry("HuggingFaceValidatorFunctor::map");
            let ParsedFile(ast) = input;

            // Convert AST back to Rust source code
            let source_code = prettyplease::unparse(&ast);

            // Create a temporary file for the Rust source code
            let temp_source_file = NamedTempFile::new()
                .context("Failed to create temporary source file")?;
            tokio::fs::write(temp_source_file.path(), source_code.as_bytes()).await
                .context("Failed to write source code to temporary file")?;
            let temp_source_file_path = temp_source_file.path().to_path_buf();

            // Create a temporary directory for the output of hf-dataset-validator
            let temp_output_dir = tempdir()
                .context("Failed to create temporary output directory")?;
            let output_path = temp_output_dir.path().to_path_buf();

            // Construct the command to execute hf-validator
            let status = tokio::process::Command::new("hf-validator")
                .arg("analyze-rust-to-ir")
                .arg(temp_source_file_path.as_os_str())
                .arg(output_path.as_os_str())
                .status().await
                .context("Failed to execute hf-validator command")?;

            if !status.success() {
                return Err(anyhow::anyhow!("hf-validator command failed with status: {}", status));
            }

            writer.write_all(format!("  -> Hugging Face Validation Result: Dataset generated at {:#?}\n", output_path).as_bytes()).await?;

            // The temporary source file will be automatically deleted when temp_source_file goes out of scope
            // The temporary output directory will be automatically deleted when temp_output_dir goes out of scope
            // We need to prevent the temporary output directory from being deleted if we want to return its path.
            // So, we'll leak the temp_output_dir for now, or copy its contents.
            // In a real scenario, the output would likely be moved to a persistent location.
            std::mem::forget(temp_output_dir); // Prevent deletion of the temporary directory

            let __result = Ok(ValidatedFile(output_path));
            measurement::record_function_exit("HuggingFaceValidatorFunctor::map");
            __result
        }
    }
}
