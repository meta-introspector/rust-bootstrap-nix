pub trait IsLlvmConfig {
    fn get_llvmconfig_name(&self) -> &'static str;
}
