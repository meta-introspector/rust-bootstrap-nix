use crate::parsed_config::ParsedConfig;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DryRun {
    Disabled,
    SelfCheck,
    UserSelected,
}
impl Default for DryRun {
    fn default() -> Self {
        DryRun::Disabled
    }
}

pub fn dry_run(config: &ParsedConfig) -> bool {
    match config.dry_run {
        DryRun::Disabled => false,
        DryRun::SelfCheck | DryRun::UserSelected => true,
    }
}
