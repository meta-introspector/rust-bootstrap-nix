use crate::prelude::*;
use crate::Build;
use crate::TargetSelection;

/// A structure representing a Rust compiler.
///
/// Each compiler has a `stage` that it is associated with and a `host` that
/// corresponds to the platform the compiler runs on. This structure is used as
/// a parameter to many methods below.
#[derive(Eq, PartialOrd, Ord, PartialEq, Clone, Copy, Hash, Debug)]
pub struct Compiler {
    pub stage: u32,
    pub host: TargetSelection,
}

impl Compiler {
    pub fn with_stage(mut self, stage: u32) -> Compiler {
        self.stage = stage;
        self
    }

    /// Returns `true` if this is a snapshot compiler for `build`'s configuration
    pub fn is_snapshot(&self, build: &Build) -> bool {
        self.stage == 0 && self.host == build.build
    }
}
