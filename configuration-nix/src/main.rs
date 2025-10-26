use crate::prelude::*;
pub mod prelude;
//mod prelude;
mod config_generator;
mod config_params;
fn main() {
    let params = config_params::ConfigParams::parse();
    config_generator::generate_config_toml(&params);
}
