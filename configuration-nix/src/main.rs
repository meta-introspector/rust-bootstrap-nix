

use clap::Parser;
mod prelude;
mod config_generator;
mod config_params;
use crate::prelude::*;

fn main() {
    let params = config_params::ConfigParams::parse();
    config_generator::generate_config_toml(&params);
}
