pub trait IsBuildConfig {
    fn get_buildconfig_name(&self) -> &'static str;
}
