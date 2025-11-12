use super::generated_command_traits::{CommandLsTrait, CommandMyLocalScriptShTrait};

// Mock implementation for 'ls' command
pub struct MockLsCommand {
    pub mock_output: Result<String, String>,
    pub expected_args: Option<Vec<String>>,
}

impl CommandLsTrait for MockLsCommand {
    fn execute(&self, args: &[&str]) -> Result<String, String> {
        if let Some(expected) = &self.expected_args {
            assert_eq!(args.to_vec(), expected.iter().map(|s| s.as_str()).collect::<Vec<&str>>());
        }
        self.mock_output.clone()
    }
}

// Mock implementation for './my_local_script.sh' command
pub struct MockMyLocalScriptCommand {
    pub mock_output: Result<String, String>,
    pub expected_args: Option<Vec<String>>,
}

impl CommandMyLocalScriptShTrait for MockMyLocalScriptCommand {
    fn execute(&self, args: &[&str]) -> Result<String, String> {
        if let Some(expected) = &self.expected_args {
            assert_eq!(args.to_vec(), expected.iter().map(|s| s.as_str()).collect::<Vec<&str>>());
        }
        self.mock_output.clone()
    }
}
