use bootstrap::Config;
use std::process::Command;
use build_helper;

#[deprecated = "use `Builder::try_run` instead where possible"]
pub(crate) fn try_run(config: &Config, cmd: &mut Command) -> Result<(), ()> {
    if config.dry_run {
        return Ok(());
    }
    config.verbose(|| println!("running: {cmd:?}"));
    build_helper::util::try_run(cmd, config.is_verbose())
}
