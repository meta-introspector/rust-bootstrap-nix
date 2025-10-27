use std::fs;


/// This module contains integral components of the build and configuration process, providing
/// support for a wide range of tasks and operations such as caching, tarballs, release
/// channels, job management, etc.

pub(crate) mod cache;
pub(crate) mod cc_detect;
pub(crate) mod change_tracker;
pub(crate) mod channel;
pub(crate) mod exec;
pub mod helpers;
pub(crate) mod job;
#[cfg(feature = "build-metrics")]
pub(crate) mod metrics;
pub(crate) mod render_tests;
pub(crate) mod shared_helpers;
pub(crate) mod tarball;

use std::path::Path;
use std::sync::OnceLock;
//use crate::BuildConfig;

use sha2::Digest;
use crate::utils::helpers::hex_encode;
use crate::t;

#[cfg(unix)]
fn chmod(path: &Path, perms: u32) {
    use std::os::unix::fs::*;
    t!(fs::set_permissions(path, fs::Permissions::from_mode(perms)));
}
#[cfg(windows)]
fn chmod(_path: &Path, _perms: u32) {}

pub fn envify(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '-' => '_',
            c => c,
        })
        .flat_map(|c| c.to_uppercase())
        .collect()
}

pub fn generate_smart_stamp_hash(
    builder: &Builder<'_>,
    dir: &Path,
    additional_input: &str,
) -> String {
    let diff = helpers::git(Some(dir))
        .allow_failure()
        .arg("diff")
        .run_capture_stdout(builder)
        .stdout_if_ok()
        .unwrap_or_default();

    let status = helpers::git(Some(dir))
        .allow_failure()
        .arg("status")
        .arg("--porcelain")
        .arg("-z")
        .arg("--untracked-files=normal")
        .run_capture_stdout(builder)
        .stdout_if_ok()
        .unwrap_or_default();

    let mut hasher = sha2::Sha256::new();

    hasher.update(diff);
    hasher.update(status);
    hasher.update(additional_input);

    hex_encode(hasher.finalize())
}

pub fn prepare_behaviour_dump_dir(build: &Build) {
    static INITIALIZED: OnceLock<bool> = OnceLock::new();

    let dump_path = build.out.join("bootstrap-shims-dump");

    let initialized = INITIALIZED.get().unwrap_or(&false);
    if !initialized {
        // clear old dumps
        if dump_path.exists() {
            t!(fs::remove_dir_all(&dump_path));
        }

        t!(fs::create_dir_all(&dump_path));

        t!(INITIALIZED.set(true));
    }
}
