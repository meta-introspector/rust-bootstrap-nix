pub fn read_commit_info_file(_path: &std::path::Path) -> String {
    "placeholder_commit_info".to_string()
}

#[derive(Default, Clone)]
pub struct GitInfo;

impl GitInfo {
    pub fn new(_omit_git_hash: bool, _path: &std::path::Path) -> Self {
        GitInfo
    }
    pub fn is_managed_git_subrepository(&self) -> bool {
        false
    }
}