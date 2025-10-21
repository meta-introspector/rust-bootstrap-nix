use std::path::PathBuf;
use crate::ParsedConfig;
use crate::LocalTomlConfig;
use crate::ConfigApplicator;
use serde::Deserialize;


#[derive(Debug, Default, Deserialize)]
pub struct Install {
    pub prefix: Option<PathBuf>,
    pub sysconfdir: Option<PathBuf>,
    pub datadir: Option<PathBuf>,
    pub docdir: Option<PathBuf>,
    pub bindir: Option<PathBuf>,
    pub libdir: Option<PathBuf>,
    pub mandir: Option<PathBuf>,
}

pub struct InstallConfigApplicator;

impl ConfigApplicator for InstallConfigApplicator {
    fn apply_to_config(&self, config: &mut ParsedConfig, toml: &LocalTomlConfig) {
        if let Some(install) = &toml.install {
            let Install { prefix, sysconfdir, docdir, bindir, libdir, mandir, datadir } = install;
            config.prefix = prefix.clone().map(PathBuf::from);
            config.sysconfdir = sysconfdir.clone().map(PathBuf::from);
            config.datadir = datadir.clone().map(PathBuf::from);
            config.docdir = docdir.clone().map(PathBuf::from);
            // Handle bindir specifically, as it's not an Option in Config
            if let Some(b) = bindir {
                config.bindir = Some(PathBuf::from(b.clone()));
            } else if let Some(p) = &config.prefix {
                config.bindir = Some(p.join("bin"));
            }
            config.libdir = libdir.clone().map(PathBuf::from);
            config.mandir = mandir.clone().map(PathBuf::from);
        }
    }
}
