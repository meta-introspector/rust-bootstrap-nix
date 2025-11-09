pub trait IsInstallConfig {
    fn get_installconfig_name(&self) -> &'static str;
}
