/// Facilitates the management and generation of tarballs.
///
/// Tarballs efficiently hold Rust compiler build artifacts and
/// capture a snapshot of each bootstrap stage.
/// In uplifting, a tarball from Stage N captures essential components
/// to assemble Stage N + 1 compiler.
use std::path::{Path, PathBuf};
