use crate::ParsedConfig;
use crate::LocalFlags;

pub fn parse_inner_flags(config: &mut ParsedConfig, flags: &mut LocalFlags) {
    config.cmd = flags.subcommand.take();
    config.incremental = flags.incremental;
    config.dry_run = flags.dry_run;
    config.verbose = Some(flags.verbose);
    config.stage = flags.stage.unwrap_or_default();
}
