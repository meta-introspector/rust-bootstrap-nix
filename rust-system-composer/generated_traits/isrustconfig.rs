pub trait IsRustConfig {
    fn get_rustconfig_name(&self) -> &'static str;
}
