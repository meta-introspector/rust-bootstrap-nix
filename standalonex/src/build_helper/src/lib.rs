//! The `build_helper` crate provides utility functions and types used by the
//! Rust build system.
//!
//! This crate is not intended for direct use by end-users.

#![deny(warnings)]

pub mod ci;
pub mod drop_bomb;
pub mod git;
pub mod metrics;
pub mod stage0_parser;
pub mod util;
pub mod channel;
pub mod llvm;
pub mod helpers;

/// The default set of crates for opt-dist to collect LLVM profiles.
pub const LLVM_PGO_CRATES: &[&str] = &[
    "syn-1.0.89",
    "cargo-0.60.0",
    "serde-1.0.136",
    "ripgrep-13.0.0",
    "regex-1.5.5",
    "clap-3.1.6",
    "hyper-0.14.18",
];

/// The default set of crates for opt-dist to collect rustc profiles.
pub const RUSTC_PGO_CRATES: &[&str] = &[
    "externs",
    "ctfe-stress-5",
    "cargo-0.60.0",
    "token-stream-stress",
    "match-stress",
    "tuple-stress",
    "diesel-1.4.8",
    "bitmaps-3.1.0",
];

pub fn get_builder_toml() -> String {
    "placeholder_builder_toml".to_string()
}
pub const RUSTC_IF_UNCHANGED_ALLOWED_PATHS: &[&str] = &[];

pub use crate::channel::GitInfo;
pub use crate::util::output;
pub use crate::channel::GitInfo as ChannelGitInfo;

pub mod prelude;