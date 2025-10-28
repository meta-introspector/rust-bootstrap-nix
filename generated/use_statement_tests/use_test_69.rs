/// In some cases, parts of bootstrap need to change part of a target spec just for one or a few
/// steps. Adding these targets to rustc proper would "leak" this implementation detail of
/// bootstrap, and would make it more complex to apply additional changes if the need arises.
///
/// To address that problem, this module implements support for "synthetic targets". Synthetic
/// targets are custom target specs generated using builtin target specs as their base. You can use
/// one of the target specs already defined in this module, or create new ones by adding a new step
/// that calls create_synthetic_target.
use crate::Compiler;
