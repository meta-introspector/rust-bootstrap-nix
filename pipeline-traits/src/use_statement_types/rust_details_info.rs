use std::fmt::Debug;
use std::path::PathBuf;

pub trait RustDetailsInfo: Send + Sync + Debug {
    fn crate_name(&self) -> Option<&str>;
    fn version(&self) -> Option<&str>;
    fn internal_external(&self) -> Option<&str>; // "internal", "external"
    fn local_mount_path(&self) -> Option<&PathBuf>;
    fn rust_version(&self) -> Option<&str>; // New field for Rust version
}

#[derive(Debug, Clone)]
pub struct RustDetails {
    pub name: Option<String>,
    pub version: Option<String>,
    pub internal_external: Option<String>,
    pub local_mount_path: Option<PathBuf>,
    pub rust_version: Option<String>, // New field
}

impl RustDetailsInfo for RustDetails {
    fn crate_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
    fn internal_external(&self) -> Option<&str> {
        self.internal_external.as_deref()
    }
    fn local_mount_path(&self) -> Option<&PathBuf> {
        self.local_mount_path.as_ref()
    }
    fn rust_version(&self) -> Option<&str> {
        self.rust_version.as_deref()
    }
}