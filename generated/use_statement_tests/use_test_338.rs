/// `./x.py clean`
///
/// Responsible for cleaning out a build directory of all old and stale
/// artifacts to prepare for a fresh build. Currently doesn't remove the
/// `build/cache` directory (download cache) or the `build/$target/llvm`
/// directory unless the `--all` flag is present.
use std::fs;
