use std::fmt::Debug;

pub trait GitInfo: Send + Sync + Debug {
    fn git_repo(&self) -> Option<&str>;
    fn git_path(&self) -> Option<&str>;
    fn our_fork_github(&self) -> Option<&str>;
    fn our_branch(&self) -> Option<&str>;
}

#[derive(Debug, Clone)]
pub struct GitDetails {
    pub repo: Option<String>,
    pub path: Option<String>,
    pub fork_github: Option<String>,
    pub branch: Option<String>,
}

impl GitInfo for GitDetails {
    fn git_repo(&self) -> Option<&str> {
        self.repo.as_deref()
    }
    fn git_path(&self) -> Option<&str> {
        self.path.as_deref()
    }
    fn our_fork_github(&self) -> Option<&str> {
        self.fork_github.as_deref()
    }
    fn our_branch(&self) -> Option<&str> {
        self.branch.as_deref()
    }
}
