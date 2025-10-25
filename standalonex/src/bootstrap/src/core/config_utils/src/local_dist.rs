
use serde_derive::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize)]
#[derive(Clone)]
pub struct LocalDist {
    pub sign_folder: Option<PathBuf>,
    pub upload_addr: Option<String>,
    pub compression_formats: Option<Vec<String>>,
    pub compression_profile: Option<String>,
    pub src_tarball: Option<bool>,
    pub include_mingw_linker: Option<bool>,
    pub vendor: Option<bool>,
}
