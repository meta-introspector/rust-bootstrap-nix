use anyhow::Result;
use crate::pipeline::UseStatement;
use crate::use_extractor;
use std::path::Path;
use crate::code_generator;
use std::fs;

// Categories
#[derive(Debug)]
pub struct RawFile(pub String, pub String);
pub struct ParsedFile(pub syn::File);
#[derive(Debug)]
pub struct UseStatements(pub Vec<String>);
#[derive(Debug)]
pub struct ClassifiedUseStatements(pub Vec<UseStatement>);

// Functors (as a trait)
pub trait PipelineFunctor<Input, Output> {
    fn map(&self, input: Input) -> Result<Output>;
}

// ParseFunctor
pub struct ParseFunctor;

impl PipelineFunctor<RawFile, ParsedFile> for ParseFunctor {
    fn map(&self, input: RawFile) -> Result<ParsedFile> {
        let RawFile(file_path, content) = input;
        match syn::parse_file(&content) {
            Ok(ast) => Ok(ParsedFile(ast)),
            Err(_) => {
                let rustc_info = use_extractor::get_rustc_info()?;
                let cache_dir = Path::new(".").join(".prelude_cache");
                let ast = use_extractor::expand_macros_and_parse(
                    Path::new(&file_path),
                    &content,
                    &rustc_info,
                    &cache_dir,
                )?;
                Ok(ParsedFile(ast))
            }
        }
    }
}

// ExtractUsesFunctor
pub struct ExtractUsesFunctor;

impl PipelineFunctor<ParsedFile, UseStatements> for ExtractUsesFunctor {
    fn map(&self, input: ParsedFile) -> Result<UseStatements> {
        let ParsedFile(ast) = input;
        let mut use_statements = Vec::new();
        for item in ast.items {
            if let syn::Item::Use(use_item) = item {
                use_statements.push(code_generator::use_item_to_string(&use_item));
            }
        }
        Ok(UseStatements(use_statements))
    }
}

// ClassifyUsesFunctor
pub struct ClassifyUsesFunctor;

impl PipelineFunctor<UseStatements, ClassifiedUseStatements> for ClassifyUsesFunctor {
    fn map(&self, input: UseStatements) -> Result<ClassifiedUseStatements> {
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
        Ok(ClassifiedUseStatements(classified_uses))
    }
}

// PreprocessFunctor
pub struct PreprocessFunctor;

impl PipelineFunctor<ClassifiedUseStatements, ClassifiedUseStatements> for PreprocessFunctor {
    fn map(&self, input: ClassifiedUseStatements) -> Result<ClassifiedUseStatements> {
        let ClassifiedUseStatements(classified_uses) = input;
        let mut new_classified_uses = Vec::new();
        for use_statement in classified_uses {
            if use_statement.error.is_some() {
                let temp_dir = tempfile::tempdir()?;
                let temp_file_path = temp_dir.path().join("main.rs");
                let content = format!("{}\nfn main() {{}}", use_statement.statement);
                fs::write(&temp_file_path, content)?;

                let output = std::process::Command::new("rustc")
                    .arg(&temp_file_path)
                    .output()?;

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
        Ok(ClassifiedUseStatements(new_classified_uses))
    }
}
