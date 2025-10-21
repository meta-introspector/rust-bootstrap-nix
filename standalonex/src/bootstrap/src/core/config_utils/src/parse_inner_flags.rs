use bootstrap::Config;
use bootstrap::Flags;
use bootstrap::DryRun;

pub fn parse_inner_flags(config: &mut Config, flags: &mut Flags) {
    config.paths = std::mem::take(&mut flags.paths);
    config.skip = flags.skip.into_iter().chain(flags.exclude).collect();
    config.include_default_paths = flags.include_default_paths;
    config.rustc_error_format = flags.rustc_error_format;
    config.json_output = flags.json_output;
    config.on_fail = flags.on_fail;
    config.cmd = flags.cmd;
    config.incremental = flags.incremental;
    config.dry_run = if flags.dry_run { DryRun::UserSelected } else { DryRun::Disabled };
    config.dump_bootstrap_shims = flags.dump_bootstrap_shims;
    config.keep_stage = flags.keep_stage;
    config.keep_stage_std = flags.keep_stage_std;
    config.color = flags.color;
    config.free_args = std::mem::take(&mut flags.free_args);
    config.llvm_profile_use = flags.llvm_profile_use;
    config.llvm_profile_generate = flags.llvm_profile_generate;
    config.enable_bolt_settings = flags.enable_bolt_settings;
    config.bypass_bootstrap_lock = flags.bypass_bootstrap_lock;
}
