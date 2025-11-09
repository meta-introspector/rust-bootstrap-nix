pub trait IsConfig {
    fn get_config_name(&self) -> &'static str;
}
