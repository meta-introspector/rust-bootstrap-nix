use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct RustcToolInfo {
    pub invocation_method: String, // e.g., "dummy", "serde", "command", "so", "static"
    pub rustc_path: Option<String>,
    pub cargo_path: Option<String>,
    pub target_triple: Option<String>,
    pub sysroot: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RustcToolDetails {
    Info(RustcToolInfo),
    Error(String),
    Unknown,
}

pub trait RustcToolInfoTrait: Send + Sync + Debug {
    fn invocation_method(&self) -> Option<&str>;
    fn rustc_path(&self) -> Option<&str>;
    fn cargo_path(&self) -> Option<&str>;
    fn target_triple(&self) -> Option<&str>;
    fn sysroot(&self) -> Option<&str>;
}

impl RustcToolInfoTrait for RustcToolDetails {
    fn invocation_method(&self) -> Option<&str> {
        match self {
            RustcToolDetails::Info(info) => Some(&info.invocation_method),
            _ => None,
        }
    }

    fn rustc_path(&self) -> Option<&str> {
        match self {
            RustcToolDetails::Info(info) => info.rustc_path.as_deref(),
            _ => None,
        }
    }

    fn cargo_path(&self) -> Option<&str> {
        match self {
            RustcToolDetails::Info(info) => info.cargo_path.as_deref(),
            _ => None,
        }
    }

    fn target_triple(&self) -> Option<&str> {
        match self {
            RustcToolDetails::Info(info) => info.target_triple.as_deref(),
            _ => None,
        }
    }

    fn sysroot(&self) -> Option<&str> {
        match self {
            RustcToolDetails::Info(info) => info.sysroot.as_deref(),
            _ => None,
        }
    }
}
