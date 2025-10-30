use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct NixInfo {
    pub flake_path: String,
    pub output_type: String,
}

#[derive(Debug, Clone)]
pub enum NixDetails {
    Info(NixInfo),
    Error(String),
    Unknown,
}

pub trait NixInfoTrait: Send + Sync + Debug {
    fn nix_flake_path(&self) -> Option<&str>;
    fn nix_output_type(&self) -> Option<&str>;
}

impl NixInfoTrait for NixDetails {
    fn nix_flake_path(&self) -> Option<&str> {
        match self {
            NixDetails::Info(info) => Some(&info.flake_path),
            _ => None,
        }
    }
    fn nix_output_type(&self) -> Option<&str> {
        match self {
            NixDetails::Info(info) => Some(&info.output_type),
            _ => None,
        }
    }
}
