pub trait IsFlakeGeneratorConfig {
    fn get_flakegeneratorconfig_name(&self) -> &'static str;
}
