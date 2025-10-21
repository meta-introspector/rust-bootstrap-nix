use bootstrap::Config;
use bootstrap::DryRun;

pub fn dry_run(config: &Config) -> bool {
    match config.dry_run {
        DryRun::Disabled => false,
        DryRun::SelfCheck | DryRun::UserSelected => true,
    }
}
