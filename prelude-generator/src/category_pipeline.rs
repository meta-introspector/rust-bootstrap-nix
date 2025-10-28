use anyhow::Result;
use crate::pipeline::UseStatement;
use crate::use_extractor;
use std::path::Path;
use crate::code_generator;
use std::fs;

use std::fmt::Debug;
use crate::measurement;
use self::{ParsedFile, ValidatedFile};

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

impl<'a, T: Debug + Clone> PipelineFunctor<T, T> for InspectFunctor<'a, T> {
    fn map(&self, input: T) -> Result<T> {
        println!("--- Inspecting: {} ---", self.label);
        println!("{:#?}", input);
        Ok(input)
    }
}
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
        measurement::record_function_entry("ParseFunctor::map");
        let RawFile(file_path, content) = input;
        let __result = match syn::parse_file(&content) {
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
        };
        measurement::record_function_exit("ParseFunctor::map");
        __result
    }
}

// ExtractUsesFunctor
pub struct ExtractUsesFunctor;

impl PipelineFunctor<ParsedFile, UseStatements> for ExtractUsesFunctor {
    fn map(&self, input: ParsedFile) -> Result<UseStatements> {
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

// ClassifyUsesFunctor
pub struct ClassifyUsesFunctor;

impl PipelineFunctor<UseStatements, ClassifiedUseStatements> for ClassifyUsesFunctor {
    fn map(&self, input: UseStatements) -> Result<ClassifiedUseStatements> {
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

// PreprocessFunctor
pub struct PreprocessFunctor;

impl PipelineFunctor<ClassifiedUseStatements, ClassifiedUseStatements> for PreprocessFunctor {
    fn map(&self, input: ClassifiedUseStatements) -> Result<ClassifiedUseStatements> {
        measurement::record_function_entry("PreprocessFunctor::map");
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
        let __result = Ok(ClassifiedUseStatements(new_classified_uses));
        measurement::record_function_exit("PreprocessFunctor::map");
        __result
    }
}

#[derive(Debug)]
pub struct ValidatedFile(pub String); // Placeholder for validation result

// HuggingFaceValidatorFunctor
pub struct HuggingFaceValidatorFunctor;

impl PipelineFunctor<self::ParsedFile, self::ValidatedFile> for HuggingFaceValidatorFunctor {
    fn map(&self, input: self::ParsedFile) -> Result<self::ValidatedFile> {
        measurement::record_function_entry("HuggingFaceValidatorFunctor::map");
        let self::ParsedFile(ast) = input;
        // Placeholder for calling hugging-face-dataset-validator-rust
        println!("Performing Hugging Face validation on parsed file.");
        let validation_result = format!("Validation successful for AST: {:?}", ast);
        let __result = Ok(self::ValidatedFile(validation_result));
        measurement::record_function_exit("HuggingFaceValidatorFunctor::map");
        __result
    }
}
