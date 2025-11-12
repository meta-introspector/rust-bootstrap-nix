use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::Path;
use tokio::process::{Command, Output};

use crate::traits::command_executor::CommandExecutor;

pub struct RealCommandExecutor;

#[async_trait]
impl CommandExecutor for RealCommandExecutor {
    async fn execute_command(&self, command: &str, args: &[&str], current_dir: Option<&Path>) -> Result<Output> {
        let mut cmd = Command::new(command);
        cmd.args(args);
        if let Some(dir) = current_dir {
            cmd.current_dir(dir);
        }
        cmd.output()
            .await
            .context(format!("Failed to execute command: {} {:?}", command, args))
    }
}