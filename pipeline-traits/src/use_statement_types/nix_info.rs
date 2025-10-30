use std::fmt::Debug;

pub trait NixInfo: Send + Sync + Debug {
    fn nix_flake_path(&self) -> Option<&str>;
}

#[derive(Debug, Clone)]
pub struct NixDetails {
    pub flake_path: Option<String>,
}

impl NixInfo for NixDetails {
    fn nix_flake_path(&self) -> Option<&str> {
        self.flake_path.as_deref()
    }
}
