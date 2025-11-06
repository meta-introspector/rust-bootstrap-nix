use crate::config_parser::Config;

pub fn generate_module_exports(config: &Config) -> String {
    let mut exports = String::new();

    if let Some(module_exports_config) = &config.module_exports {
        if let Some(modules) = &module_exports_config.modules {
            for module_path in modules {
                exports.push_str(&format!("pub use {};\n", module_path));
            }
        }
    }

    exports
}
