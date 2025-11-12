pub trait IsCommands {
    fn get_commands_name(&self) -> &'static str;
}
