use crate::prelude::*


/// When building Rust various objects are handled differently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DependencyType {
    /// Libraries originating from proc-macros.
    Host,
    /// Typical Rust libraries.
    Target,
    /// Non Rust libraries and objects shipped to ease usage of certain targets.
    TargetSelfContained,
}
