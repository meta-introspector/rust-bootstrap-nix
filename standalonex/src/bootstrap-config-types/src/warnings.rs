use crate::prelude::*;
/// Whether to deny warnings, emit them as warnings, or use the default behavior
#[derive(Copy, Clone, Default, Debug, ValueEnum)]
pub enum Warnings {
    Deny,
    Warn,
    #[default]
    Default,
}
