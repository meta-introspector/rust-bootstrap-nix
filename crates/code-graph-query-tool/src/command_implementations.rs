use super::generated_command_traits::{CommandLsTrait, CommandMyLocalScriptShTrait};

// Concrete implementation for 'ls' command
pub struct RealLsCommand;

impl CommandLsTrait for RealLsCommand {
    fn execute(&self, args: &[&str]) -> Result<String, String> {
        let output = std::process::Command::new("ls")
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute 'ls': {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(format!("'ls' command failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
}

// Concrete implementation for './my_local_script.sh' command
pub struct RealMyLocalScriptCommand;

impl CommandMyLocalScriptShTrait for RealMyLocalScriptCommand {
    fn execute(&self, args: &[&str]) -> Result<String, String> {
        let output = std::process::Command::new("./my_local_script.sh")
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute './my_local_script.sh': {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(format!("'./my_local_script.sh' command failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
}
