use anyhow::Result;
use std::path::PathBuf;
use std::pin::Pin;
use std::future::Future;
use std::boxed::Box;
use std::fmt::Debug;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub mod use_statement_types;
pub use use_statement_types::{
    GitDetails, GitInfo, GitInfoTrait,
    NixDetails, NixInfo, NixInfoTrait,
    RustDetails, RustDetailsInfo, RustDetailsInfoTrait,
    CargoDetails, CargoInfo, CargoInfoTrait,
    SynDetails, SynInfo, SynInfoTrait,
    LlvmDetails, LlvmInfo, LlvmInfoTrait,
    LinuxDetails, LinuxInfo, LinuxInfoTrait,
};

#[derive(Debug)]
pub struct RawFile(pub String, pub String);
#[derive(Clone)]
pub struct ParsedFile(pub String, pub PathBuf);
#[derive(Debug)]
pub struct UseStatements(pub Vec<String>);
#[derive(Debug)]
pub struct ClassifiedUseStatements(pub Vec<UseStatement>, pub HashMap<String, Vec<String>>);
#[derive(Debug, Clone)]
pub struct ValidatedFile(pub String, pub PathBuf);

// Functors (as a trait)
pub trait PipelineFunctor<Input: Send + 'static, Output: Send + 'static> {
    fn map<'writer>(
        &'writer self,
        writer: &'writer mut (impl tokio::io::AsyncWriteExt + Unpin + Send),
        input: Input,
    ) -> Pin<Box<dyn Future<Output = Result<Output>> + Send + 'writer>>;
}

#[derive(Debug)]
pub struct UseStatement {
    pub statement: String,
    pub error: Option<String>,
    // Composed traits
    pub git_details: Option<GitDetails>,
    pub nix_details: Option<NixDetails>,
    pub rust_details: Option<RustDetails>,
    pub cargo_details: Option<CargoDetails>,
    pub syn_details: Option<SynDetails>,
    pub llvm_details: Option<LlvmDetails>,
    pub linux_details: Option<LinuxDetails>,
}

/// Information about a variable found in the AST
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableInfo {
    pub name: String,
    pub type_name: String,
    pub is_mutable: bool,
    pub scope: String, // e.g., "function", "module", "global"
}

/// Information about a function found in the AST
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub visibility: String, // e.g., "public", "private"
    pub arg_count: u32,
    pub arg_types: Vec<String>,
    pub return_type: String,
    pub is_async: bool,
    pub is_unsafe: bool,
    pub is_const: bool,
}

/// Information about an import statement found in the AST
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportInfo {
    pub path: String, // The full path of the import (e.g., "std::collections::HashMap")
    pub alias: Option<String>,
    pub is_external: bool,
    pub source_crate: Option<String>,
    pub git_source_url: Option<String>,
    pub git_branch: Option<String>,
}

/// Comprehensive AST analysis data for a Rust project
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AstStatistics {
    pub node_type_counts: HashMap<String, u32>,
    pub variable_declarations: Vec<VariableInfo>,
    pub function_definitions: Vec<FunctionInfo>,
    pub import_statements: Vec<ImportInfo>,
    // Add more fields as needed, e.g., macro invocations, struct definitions
}
