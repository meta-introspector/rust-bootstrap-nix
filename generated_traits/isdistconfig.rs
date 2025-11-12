pub trait IsDistConfig {
    fn get_distconfig_name(&self) -> &'static str;
}
