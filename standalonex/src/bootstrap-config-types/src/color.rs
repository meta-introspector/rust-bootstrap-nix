use clap::*;
use build_helper::prelude::*;
#[derive(Copy, Clone, Default, Debug, ValueEnum)]
pub enum Color {
    Always,
    Never,
    #[default]
    Auto,
}
