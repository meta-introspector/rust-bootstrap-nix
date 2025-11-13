use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Crate {
    pub name: String,
    pub deps: HashSet<String>,
    pub path: PathBuf,
    pub has_lib: bool,
    pub features: Vec<String>,
}

impl Crate {
}
