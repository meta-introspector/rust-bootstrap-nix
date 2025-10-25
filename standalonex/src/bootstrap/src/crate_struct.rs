use crate::prelude::*


use std::collections::HashSet;
use std::path::PathBuf;

use crate::Build;

#[derive(Debug, Clone)]
pub struct Crate {
    pub name: String,
    pub deps: HashSet<String>,
    pub path: PathBuf,
    pub has_lib: bool,
    pub features: Vec<String>,
}

impl Crate {
    pub fn local_path(&self, build: &Build) -> PathBuf {
        self.path.strip_prefix(&build.config.src).unwrap().into()
    }
}
