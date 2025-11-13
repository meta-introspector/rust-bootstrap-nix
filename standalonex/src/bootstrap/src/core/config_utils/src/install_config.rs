use crate::prelude::*;
#[derive(Debug, Default, Deserialize)]
#[derive(Clone)]
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
            let Install {
                prefix,
                sysconfdir,
                docdir,
                bindir,
                libdir,
                mandir,
                datadir,
            } = install;
            config.prefix = prefix.clone();
            config.sysconfdir = sysconfdir.clone();
            config.datadir = datadir.clone();
            config.docdir = docdir.clone();
            if let Some(b) = bindir {
                config.bindir = Some(b.clone());
            } else if let Some(p) = &config.prefix {
                config.bindir = Some(p.join("bin"));
            }
            config.libdir = libdir.clone();
            config.mandir = mandir.clone();
        }
    }
}
