use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct GitInfo {
    pub repo_url: String,
    pub branch: String,
    pub commit_hash: String,
}

#[derive(Debug, Clone)]
pub enum GitDetails {
    Info(GitInfo),
    Error(String),
    Unknown,
}

// The trait is now implemented for the enum, delegating to the Info variant
pub trait GitInfoTrait: Send + Sync + Debug {
    fn git_repo(&self) -> Option<&str>;
    fn git_path(&self) -> Option<&str>;
    fn our_fork_github(&self) -> Option<&str>;
    fn our_branch(&self) -> Option<&str>;
}

impl GitInfoTrait for GitDetails {
    fn git_repo(&self) -> Option<&str> {
        match self {
            GitDetails::Info(info) => Some(&info.repo_url),
            _ => None,
        }
    }
    fn git_path(&self) -> Option<&str> {
        // Assuming git_path is part of repo_url or a separate field in GitInfo
        // For now, returning None as it's not directly in GitInfo
        None
    }
    fn our_fork_github(&self) -> Option<&str> {
        // Assuming this is part of repo_url or a separate field in GitInfo
        None
    }
    fn our_branch(&self) -> Option<&str> {
        match self {
            GitDetails::Info(info) => Some(&info.branch),
            _ => None,
        }
    }
}
