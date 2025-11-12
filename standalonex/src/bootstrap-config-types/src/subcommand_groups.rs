use build_helper::prelude::*;
#[derive(Debug, Clone, clap::Subcommand)]
pub enum QaTool {
    Check { #[arg(long)] all_targets: bool },
    Clippy {
        #[arg(long)]
        fix: bool,
        #[arg(long, requires = "fix")]
        allow_dirty: bool,
        #[arg(long, requires = "fix")]
        allow_staged: bool,
        #[arg(
            global = true,
            short = 'A',
            action = clap::ArgAction::Append,
            value_name = "LINT"
        )]
        allow: Vec<String>,
        #[arg(
            global = true,
            short = 'D',
            action = clap::ArgAction::Append,
            value_name = "LINT"
        )]
        deny: Vec<String>,
        #[arg(
            global = true,
            short = 'W',
            action = clap::ArgAction::Append,
            value_name = "LINT"
        )]
        warn: Vec<String>,
        #[arg(
            global = true,
            short = 'F',
            action = clap::ArgAction::Append,
            value_name = "LINT"
        )]
        forbid: Vec<String>,
    },
    Fix,
    Format { #[arg(long)] check: bool, #[arg(long)] all: bool },
    Test {
        #[arg(long)]
        no_fail_fast: bool,
        #[arg(long, value_name = "ARGS", allow_hyphen_values(true))]
        test_args: Vec<String>,
        #[arg(long, value_name = "ARGS", allow_hyphen_values(true))]
        compiletest_rustc_args: Vec<String>,
        #[arg(long)]
        no_doc: bool,
        #[arg(long)]
        doc: bool,
        #[arg(long)]
        bless: bool,
        #[arg(long, value_name = "check | build | run")]
        pass: Option<String>,
        #[arg(long, value_name = "auto | always | never")]
        run: Option<String>,
        #[arg(long)]
        rustfix_coverage: bool,
    },
    Miri {
        #[arg(long)]
        no_fail_fast: bool,
        #[arg(long, value_name = "ARGS", allow_hyphen_values(true))]
        test_args: Vec<String>,
        #[arg(long)]
        no_doc: bool,
        #[arg(long)]
        doc: bool,
    },
    Bench { #[arg(long, allow_hyphen_values(true))] test_args: Vec<String> },
    Suggest { #[arg(long)] run: bool },
    Perf,
}
#[derive(Debug, Clone, clap::Subcommand)]
pub enum BuildTool {
    Build,
    Doc { #[arg(long)] open: bool, #[arg(long)] json: bool },
}
#[derive(Debug, Clone, clap::Subcommand)]
pub enum DistTool {
    Dist,
    Install,
}
#[derive(Debug, Clone, clap::Subcommand)]
pub enum MiscTool {
    Clean { #[arg(long)] all: bool, #[arg(long, value_name = "N")] stage: Option<u32> },
    Run { #[arg(long, allow_hyphen_values(true))] args: Vec<String> },
    Setup { #[arg(value_name = "<PROFILE>|hook|editor|link")] profile: Option<PathBuf> },
    Vendor { #[arg(long)] sync: Vec<PathBuf>, #[arg(long)] versioned_dirs: bool },
}
