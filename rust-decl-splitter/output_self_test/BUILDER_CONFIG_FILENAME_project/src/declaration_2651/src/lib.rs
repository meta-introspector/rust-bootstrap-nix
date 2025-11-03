#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

# [doc = " This file is embedded in the overlay directory of the tarball sources. It is"] # [doc = " useful in scenarios where developers want to see how the tarball sources were"] # [doc = " generated."] # [doc = ""] # [doc = " We also use this file to compare the host's config.toml against the CI rustc builder"] # [doc = " configuration to detect any incompatible options."] pub const BUILDER_CONFIG_FILENAME : & str = "builder-config" ;