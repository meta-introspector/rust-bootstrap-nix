use std::path::Path;

pub fn git(_path: Option<&Path>) -> GitCommand {
    GitCommand
}

pub struct GitCommand;

impl GitCommand {
    pub fn allow_failure(self) -> Self {
        self
    }
    pub fn as_command_mut(&mut self) -> &mut std::process::Command {
        unimplemented!()
    }
}

pub fn dir_is_empty(_path: &Path) -> bool {
    false
}