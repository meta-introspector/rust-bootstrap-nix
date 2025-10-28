/// Sanity checking performed by bootstrap before actually executing anything.
///
/// This module contains the implementation of ensuring that the build
/// environment looks reasonable before progressing. This will verify that
/// various programs like git and python exist, along with ensuring that all C
/// compilers for cross-compiling are found.
///
/// In theory if we get past this phase it's a bug if a build fails, but in
/// practice that's likely not true!
use std::collections::{HashMap, HashSet};
