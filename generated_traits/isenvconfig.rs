pub trait IsEnvConfig {
    fn get_envconfig_name(&self) -> &'static str;
}
