pub trait IsNixConfig {
    fn get_nixconfig_name(&self) -> &'static str;
}
