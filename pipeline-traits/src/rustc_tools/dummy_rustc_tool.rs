use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::path::PathBuf;

use crate::rustc_tool_trait::RustcToolTrait;

pub struct DummyRustcTool;

#[async_trait]
impl RustcToolTrait for DummyRustcTool {
    async fn compile(&self, _input_path: &PathBuf, _output_path: &PathBuf) -> Result<()> {
        println!("DummyRustcTool: Compiling (no-op)");
        Ok(())
    }

    async fn check(&self, _input_path: &PathBuf) -> Result<()> {
        println!("DummyRustcTool: Checking (no-op)");
        Ok(())
    }

    async fn version(&self) -> Result<String> {
        println!("DummyRustcTool: Getting version (mock)");
        Ok("Dummy rustc version 1.0.0".to_string())
    }

    async fn run_command(&self, args: &[&str]) -> Result<String> {
        println!("DummyRustcTool: Running command (no-op) with args: {:?}", args);
        Ok(format!("Dummy command output for args: {:?}", args))
    }
}
