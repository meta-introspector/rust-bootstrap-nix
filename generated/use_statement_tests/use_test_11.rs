/// Build configuration for Rust's release channels.
///
/// Implements the stable/beta/nightly channel distinctions by setting various
/// flags like the `unstable_features`, calculating variables like `release` and
/// `package_vers`, and otherwise indicating to the compiler what it should
/// print out as part of its version information.
use std::fs;
