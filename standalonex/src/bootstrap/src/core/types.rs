use crate::prelude::*;


use std::fmt::Debug;
use std::hash::Hash;

//use crate::builder::Builder;
//use crate::{Subcommand, TargetSelection, Kind, Compiler};

pub trait RustcTaskConfig: Sized + Debug + Clone + PartialEq + Eq + Hash {
    fn default_config(builder: &Builder<'_>) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rustc<C: RustcTaskConfig> {
    pub compiler: Compiler,
    pub target: TargetSelection,
    pub crates: Vec<String>,
    pub config: C, // Task-specific configuration
}

pub trait StdTaskConfig: Sized + Debug + Clone + PartialEq + Eq + Hash {
    fn get_crates(&self) -> &Vec<String>;
    fn get_override_build_kind(&self) -> Option<Kind>;
    fn default_config(builder: &Builder<'_>) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Std<C: StdTaskConfig> {
    pub target: TargetSelection,
    pub config: C,
    pub crates: Vec<String>,
}

// Concrete implementations of StdTaskConfig
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CheckStdConfig {
    pub override_build_kind: Option<Kind>,
}

impl CheckStdConfig {
    pub fn new(override_build_kind: Option<Kind>) -> Self {
        Self { override_build_kind }
    }
}

impl StdTaskConfig for CheckStdConfig {
    fn get_crates(&self) -> &Vec<String> {
        &vec![]
    }
    fn get_override_build_kind(&self) -> Option<Kind> {
        self.override_build_kind
    }
    fn default_config(_builder: &Builder<'_>) -> Self {
        Self::new(None)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClippyStdConfig {
    pub config: LintConfig,
}

impl ClippyStdConfig {
    pub fn new(config: LintConfig) -> Self {
        Self { config }
    }
}

impl StdTaskConfig for ClippyStdConfig {
    fn get_crates(&self) -> &Vec<String> {
        &vec![]
    }
    fn get_override_build_kind(&self) -> Option<Kind> {
        None
    }
    fn default_config(builder: &Builder<'_>) -> Self {
        Self::new(LintConfig::new(builder))
    }
}



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CheckRustcConfig {
    pub override_build_kind: Option<Kind>,
}

impl CheckRustcConfig {
    pub fn new(override_build_kind: Option<Kind>) -> Self {
        Self { override_build_kind }
    }
}

impl RustcTaskConfig for CheckRustcConfig {
    fn default_config(_builder: &Builder<'_>) -> Self {
        Self::new(None)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LintConfig {
    pub allow: Vec<String>,
    pub warn: Vec<String>,
    pub deny: Vec<String>,
    pub forbid: Vec<String>,
}

impl LintConfig {
    pub fn new(builder: &Builder<'_>) -> Self {
        match builder.config.cmd.clone() {
            Subcommand::Clippy { allow, deny, warn, forbid, .. } => {
                Self { allow, warn, deny, forbid }
            }
            _ => unreachable!("LintConfig can only be called from `clippy` subcommands."),
        }
    }

    pub fn merge(&self, other: &Self) -> Self {
        let merged = |self_attr: &[String], other_attr: &[String]| -> Vec<String> {
            self_attr.iter().cloned().chain(other_attr.iter().cloned()).collect()
        };
        // This is written this way to ensure we get a compiler error if we add a new field.
        Self {
            allow: merged(&self.allow, &other.allow),
            warn: merged(&self.warn, &other.warn),
            deny: merged(&self.deny, &other.deny),
            forbid: merged(&self.forbid, &other.forbid),
        }
    }
}

impl RustcTaskConfig for LintConfig {
    fn default_config(builder: &Builder<'_>) -> Self {
        Self::new(builder)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompileRustcConfig {
    // No specific fields needed if compiler, target, crates are in generic Rustc
}

impl RustcTaskConfig for CompileRustcConfig {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DistRustcConfig {
    // No specific fields needed if compiler is in generic Rustc
}

impl RustcTaskConfig for DistRustcConfig {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocRustcConfig {
    pub stage: u32,
    pub validate: bool,
}

impl RustcTaskConfig for DocRustcConfig {}
