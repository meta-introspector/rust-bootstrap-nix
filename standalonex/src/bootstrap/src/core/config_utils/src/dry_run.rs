use crate::parsed_config::ParsedConfig;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[derive(Default)]
pub enum DryRun {
    #[default]
    Disabled,
    SelfCheck,
    UserSelected,
}

pub fn dry_run(config: &ParsedConfig) -> bool {
    match config.dry_run {
        DryRun::Disabled => false,
        DryRun::SelfCheck | DryRun::UserSelected => true,
    }
}
