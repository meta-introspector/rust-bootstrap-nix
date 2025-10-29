use anyhow::{Context, Result};
use crate::pipeline::UseStatement;
use crate::use_extractor;
use std::path::{Path, PathBuf};
use crate::code_generator;
//use tokio::io::AsyncWriteExt;
use std::fmt::Debug;
use crate::measurement;
use indoc::indoc;

use tempfile::tempdir;

#[derive(Debug)]
pub struct ReconstructedAst;

// AstReconstructionFunctor
pub struct AstReconstructionFunctor;

impl PipelineFunctor<ValidatedFile, ReconstructedAst> for AstReconstructionFunctor {
    fn map(&self, writer: &mut (impl tokio::io::AsyncWriteExt + Unpin + Send), input: ValidatedFile) -> impl std::future::Future<Output = Result<ReconstructedAst>> {
        async move {
            measurement::record_function_entry("AstReconstructionFunctor::map");
            let ValidatedFile(dataset_path) = input;

            writer.write_all(format!("--- Stage 5: AST Reconstruction from Hugging Face Dataset ---\n").as_bytes()).await?;
            writer.write_all(format!("  -> Dataset path: {:#?}\n", dataset_path).as_bytes()).await?;

            crate::hf_dataset_reader::reconstruct_ast_from_hf_dataset(&dataset_path).await
                .context("Failed to reconstruct AST from Hugging Face dataset")?;

            writer.write_all(format!("  -> AST Reconstruction completed successfully.\n").as_bytes()).await?;

            let __result = Ok(ReconstructedAst);
            measurement::record_function_exit("AstReconstructionFunctor::map");
            __result
        }
    }
}

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
pub struct ParsedFile(pub syn::File, pub PathBuf);
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
            let RawFile(file_path_str, content) = input;
            let file_path = PathBuf::from(file_path_str.clone());
            let __result = match syn::parse_file(&content) {
                Ok(ast) => Ok(ParsedFile(ast, file_path.clone())),
                Err(_) => {
                    let rustc_info = use_extractor::get_rustc_info()?;
                    let cache_dir = Path::new(".").join(".prelude_cache");
                    let ast = use_extractor::expand_macros_and_parse(
                        writer, // Pass the writer here
                        &file_path,
                        &content,
                        &rustc_info,
                        &cache_dir,
                    ).await?;
                    Ok(ParsedFile(ast, file_path.clone()))
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
            let ParsedFile(ast, _) = input;
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
            let ParsedFile(ast, original_file_path) = input;

            // Convert AST back to Rust source code
            let source_code = prettyplease::unparse(&ast);

            // Generate a canonical directory name based on the original_file_path
            let sanitized_file_path = original_file_path.to_string_lossy().replace("/", "_").replace(".", "_");
            writer.write_all(format!("  -> Sanitized file path for hf-validator project: {}\n", sanitized_file_path).as_bytes()).await?;
            let hf_validator_project_dir = PathBuf::from(format!("generated/hf_validator_projects/{}", sanitized_file_path));
            tokio::fs::create_dir_all(&hf_validator_project_dir).await?;

            // Write the Rust source code to a file within the persistent directory
            let source_file_path = hf_validator_project_dir.join("main.rs");
            tokio::fs::write(&source_file_path, source_code.as_bytes()).await
                .context("Failed to write source code to persistent file")?;

            // Create a minimal Cargo.toml in the persistent directory for hf-validator
            let cargo_toml_content = indoc! {
                r#"[package]
                name = "temp_hf_project"
                version = "0.1.0"
                edition = "2021"

                [[bin]]
                name = "temp_hf_project"
                path = "main.rs"

                [dependencies]
                anyhow = "1.0"
                tokio = { version = "1", features = ["full"] }

                [workspace]
                "#
            };
            tokio::fs::write(hf_validator_project_dir.join("Cargo.toml"), cargo_toml_content).await?;

            // Initialize a Git repository and make a dummy commit
            let output = tokio::process::Command::new("git")
                .arg("init")
                .current_dir(&hf_validator_project_dir)
                .output().await
                .context("Failed to initialize git repository")?;
            if !output.status.success() {
                return Err(anyhow::anyhow!("git init failed: {}", String::from_utf8_lossy(&output.stderr)));
            }

            // Configure dummy Git user and email
            let output = tokio::process::Command::new("git")
                .arg("config")
                .arg("user.email")
                .arg("test@example.com")
                .current_dir(&hf_validator_project_dir)
                .output().await
                .context("Failed to configure git user email")?;
            if !output.status.success() {
                return Err(anyhow::anyhow!("git config user.email failed: {}\nStderr: {}", String::from_utf8_lossy(&output.status.code().unwrap_or(-1).to_string().as_bytes()), String::from_utf8_lossy(&output.stderr)));
            }

            let output = tokio::process::Command::new("git")
                .arg("config")
                .arg("user.name")
                .arg("Test User")
                .current_dir(&hf_validator_project_dir)
                .output().await
                .context("Failed to configure git user name")?;
            if !output.status.success() {
                return Err(anyhow::anyhow!("git config user.name failed: {}\nStderr: {}", String::from_utf8_lossy(&output.status.code().unwrap_or(-1).to_string().as_bytes()), String::from_utf8_lossy(&output.stderr)));
            }

            // Create an initial empty commit to satisfy hf-validator's git requirements
            let output = tokio::process::Command::new("git")
                .arg("commit")
                .arg("--allow-empty")
                .arg("-m")
                .arg("Initial empty commit")
                .current_dir(&hf_validator_project_dir)
                .output().await
                .context("Failed to make initial empty git commit")?;
            if !output.status.success() {
                return Err(anyhow::anyhow!("git commit --allow-empty failed: {}\nStderr: {}", String::from_utf8_lossy(&output.status.code().unwrap_or(-1).to_string().as_bytes()), String::from_utf8_lossy(&output.stderr)));
            }

            // Create a temporary directory for the output of hf-dataset-validator
            let temp_output_dir = tempdir()
                .context("Failed to create temporary output directory")?;
            let output_path = temp_output_dir.path().to_path_buf();

            // Construct the command to execute hf-validator
            let status = tokio::process::Command::new("hf-validator")
                .arg("analyze-rust-to-ir")
                .arg(hf_validator_project_dir.as_os_str())
                .arg(output_path.as_os_str())
                .status().await
                .context("Failed to execute hf-validator command")?;

            if !status.success() {
                return Err(anyhow::anyhow!("hf-validator command failed with status: {}", status));
            }

            writer.write_all(format!("  -> Hugging Face Validation Result: Dataset generated at {:#?}\n", output_path).as_bytes()).await?;

            // Define the permanent output directory
            let permanent_output_dir = PathBuf::from(format!("generated/hf_dataset_output/{}", sanitized_file_path));
            tokio::fs::create_dir_all(&permanent_output_dir).await?;

            // Copy contents from temporary output directory to permanent directory
            // This requires iterating through the temporary directory and copying each item.
            let mut entries = tokio::fs::read_dir(&output_path).await?;
            while let Some(entry) = entries.next_entry().await? {
                let entry_path = entry.path();
                let destination_path = permanent_output_dir.join(entry_path.file_name().unwrap());
                if entry_path.is_dir() {
                    // Recursively copy directories
                    copy_dir_all(&entry_path, &destination_path).await?;
                } else {
                    tokio::fs::copy(&entry_path, &destination_path).await?;
                }
            }

            // The temporary output directory will be automatically deleted when temp_output_dir goes out of scope
            // We no longer need to forget it as we are copying its contents.

            let __result = Ok(ValidatedFile(permanent_output_dir));
            measurement::record_function_exit("HuggingFaceValidatorFunctor::map");
            __result
        }
    }
}
// Helper function to recursively copy a directory
async fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    tokio::fs::create_dir_all(&dst).await?;
    let mut stack = vec![src.as_ref().to_path_buf()];

    while let Some(current_src) = stack.pop() {
        let current_dst = dst.as_ref().join(current_src.strip_prefix(src.as_ref()).unwrap());
        let mut entries = tokio::fs::read_dir(&current_src).await?;

        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();
            let relative_path = entry_path.strip_prefix(&current_src).unwrap();
            let destination_path = current_dst.join(relative_path);

            if entry_path.is_dir() {
                tokio::fs::create_dir_all(&destination_path).await?;
                stack.push(entry_path);
            } else {
                tokio::fs::copy(&entry_path, &destination_path).await?;
            }
        }
    }
    Ok(())
}
