use crate::prelude::*;


use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::io;

use crate::Build;
use crate::flags;
use termcolor::{ColorChoice, StandardStream, WriteColor};

pub fn envify(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '-' => '_',
            c => c,
        })
        .flat_map(|c| c.to_uppercase())
        .collect()
}

#[cfg(unix)]
pub fn chmod(path: &Path, perms: u32) {
    use std::os::unix::fs::*;
    t!(fs::set_permissions(path, fs::Permissions::from_mode(perms)));
}
#[cfg(windows)]
pub fn chmod(_path: &Path, _perms: u32) {}
