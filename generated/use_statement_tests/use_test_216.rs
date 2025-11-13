/// This module serves two purposes:
///     1. It is part of the `utils` module and used in other parts of bootstrap.
///     2. It is embedded inside bootstrap shims to avoid a dependency on the bootstrap library.
///        Therefore, this module should never use any other bootstrap module. This reduces binary
///        size and improves compilation time by minimizing linking time.
#[allow(dead_code)]
use std::env;
