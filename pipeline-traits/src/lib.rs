use anyhow::Result;
use std::path::PathBuf;
use std::pin::Pin;
use std::future::Future;
use std::boxed::Box;
use std::fmt::Debug;
//use tokio::io::AsyncWriteExt;

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
pub struct ClassifiedUseStatements(pub Vec<UseStatement>);
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

#[derive(Debug, Default)]
pub struct AstStatistics {
    pub node_type_counts: std::collections::HashMap<String, usize>,
    pub line_stats: std::collections::HashMap<String, (usize, usize, usize, usize)>, // type -> (min, max, sum, count)
    pub column_stats: std::collections::HashMap<String, (usize, usize, usize, usize)>, // type -> (min, max, sum, count)
    pub processing_time_stats: std::collections::HashMap<String, (u64, u64, u64, usize)>, // type -> (min, max, sum, count)
    pub rust_version_counts: std::collections::HashMap<String, usize>,
    pub analyzer_version_counts: std::collections::HashMap<String, usize>,
    pub snippet_length_stats: std::collections::HashMap<String, (usize, usize, usize, usize)>, // type -> (min, max, sum, count)
}
