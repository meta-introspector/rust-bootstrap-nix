use build_helper::prelude::*;
/// This file is embedded in the overlay directory of the tarball sources. It is
/// useful in scenarios where developers want to see how the tarball sources were
/// generated.
///
/// We also use this file to compare the host's config.toml against the CI rustc builder
/// configuration to detect any incompatible options.
pub const BUILDER_CONFIG_FILENAME: &str = "builder-config";
#[derive(Clone, Default)]
pub enum DryRun {
    /// This isn't a dry run.
    #[default]
    Disabled,
    /// This is a dry run enabled by bootstrap itself, so it can verify that no work is done.
    SelfCheck,
    /// This is a dry run enabled by the `--dry-run` flag.
    UserSelected,
}
