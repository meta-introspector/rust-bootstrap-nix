use crate::prelude::*;


//! Command-line interface of the bootstrap build system.
//!
//! This module implements the command-line parsing of the build system which
//! has various flags to configure how it's run.

use std::path::{Path, PathBuf};

use clap::{CommandFactory, Parser, ValueEnum};

use crate::core::build_steps::setup::Profile;
use crate::core::builder::{Builder, Kind};
use crate::core::config::{Config, TargetSelectionList, target_selection_list};
use crate::{Build, DocTests};
pub use crate::core::config::subcommand::get_completion;



#[derive(Clone, ValueEnum)]
pub enum Warnings {
    Default,
    Deny,
    Warn,
}

#[derive(Clone, ValueEnum)]
pub enum Color {
    Auto,
    Always,
    Never,
}

#[derive(Debug, Parser)]
#[command(
    override_usage = "x.py <subcommand> [options] [<paths>...]",
    disable_help_subcommand(true),
    about = "",
    next_line_help(false)
)]
pub struct Flags {
    #[command(subcommand)]
    pub cmd: Subcommand,

    #[arg(global = true, short, long, action = clap::ArgAction::Count)]
    /// use verbose output (-vv for very verbose)
    pub verbose: u8, // each extra -v after the first is passed to Cargo
    #[arg(global = true, short, long)]
    /// use incremental compilation
    pub incremental: bool,
    #[arg(global = true, long, value_hint = clap::ValueHint::FilePath, value_name = "FILE")]
    /// TOML configuration file for build
    pub config: Option<PathBuf>,
    #[arg(global = true, long, value_hint = clap::ValueHint::DirPath, value_name = "DIR")]
    /// Build directory, overrides `build.build-dir` in `config.toml`
    pub build_dir: Option<PathBuf>,

    #[arg(global = true, long, value_hint = clap::ValueHint::Other, value_name = "BUILD")]
    /// build target of the stage0 compiler
    pub build: Option<String>,

    #[arg(global = true, long, value_hint = clap::ValueHint::Other, value_name = "HOST", value_parser = target_selection_list)]
    /// host targets to build
    pub host: Option<TargetSelectionList>,

    #[arg(global = true, long, value_hint = clap::ValueHint::Other, value_name = "TARGET", value_parser = target_selection_list)]
    /// target targets to build
    pub target: Option<TargetSelectionList>,

    #[arg(global = true, long, value_name = "PATH")]
    /// build paths to exclude
    pub exclude: Vec<PathBuf>, // keeping for client backward compatibility
    #[arg(global = true, long, value_name = "PATH")]
    /// build paths to skip
    pub skip: Vec<PathBuf>,
    #[arg(global = true, long)]
    /// include default paths in addition to the provided ones
    pub include_default_paths: bool,

    #[arg(global = true, value_hint = clap::ValueHint::Other, long)]
    pub rustc_error_format: Option<String>,

    #[arg(global = true, long, value_hint = clap::ValueHint::CommandString, value_name = "CMD")]
    /// command to run on failure
    pub on_fail: Option<String>,
    #[arg(global = true, long)]
    /// dry run; don't build anything
    pub dry_run: bool,
    /// Indicates whether to dump the work done from bootstrap shims
    #[arg(global = true, long)]
    pub dump_bootstrap_shims: bool,
    #[arg(global = true, value_hint = clap::ValueHint::Other, value_name = "N")]
    /// stage to build (indicates compiler to use/test, e.g., stage 0 uses the
    /// bootstrap compiler, stage 1 the stage 0 rustc artifacts, etc.)
    pub stage: Option<u32>,

    #[arg(global = true, value_hint = clap::ValueHint::Other, long, value_name = "N")]
    /// stage(s) to keep without recompiling
    /// (pass multiple times to keep e.g., both stages 0 and 1)
    pub keep_stage: Vec<u32>,
    #[arg(global = true, value_hint = clap::ValueHint::Other, long, value_name = "N")]
    /// stage(s) of the standard library to keep without recompiling
    /// (pass multiple times to keep e.g., both stages 0 and 1)
    pub keep_stage_std: Vec<u32>,
    #[arg(global = true, long, value_hint = clap::ValueHint::DirPath, value_name = "DIR")]
    /// path to the root of the rust checkout
    pub src: Option<PathBuf>,

    #[arg(
        global = true,
        short,
        long,
        value_hint = clap::ValueHint::Other,
        value_name = "JOBS"
    )]
    /// number of jobs to run in parallel
    pub jobs: Option<u32>,
    // This overrides the deny-warnings configuration option,
    // which passes -Dwarnings to the compiler invocations.
    #[arg(global = true, long)]
    #[arg(value_enum, default_value_t=Warnings::Default, value_name = "deny|warn")]
    /// if value is deny, will deny warnings
    /// if value is warn, will emit warnings
    /// otherwise, use the default configured behaviour
    pub warnings: Warnings,

    #[arg(global = true, value_hint = clap::ValueHint::Other, long, value_name = "FORMAT")]
    /// rustc error format
    pub error_format: Option<String>,
    #[arg(global = true, long)]
    /// use message-format=json
    pub json_output: bool,

    #[arg(global = true, long, value_name = "STYLE")]
    #[arg(value_enum, default_value_t = Color::Auto)]
    /// whether to use color in cargo and rustc output
    pub color: Color,

    #[arg(global = true, long)]
    /// Bootstrap uses this value to decide whether it should bypass locking the build process.
    /// This is rarely needed (e.g., compiling the std library for different targets in parallel).
    ///
    /// Unless you know exactly what you are doing, you probably don't need this.
    pub bypass_bootstrap_lock: bool,

    /// generate PGO profile with rustc build
    #[arg(global = true, value_hint = clap::ValueHint::FilePath, long, value_name = "PROFILE")]
    pub rust_profile_generate: Option<String>,
    /// use PGO profile for rustc build
    #[arg(global = true, value_hint = clap::ValueHint::FilePath, long, value_name = "PROFILE")]
    pub rust_profile_use: Option<String>,
    /// use PGO profile for LLVM build
    #[arg(global = true, value_hint = clap::ValueHint::FilePath, long, value_name = "PROFILE")]
    pub llvm_profile_use: Option<String>,
    // LLVM doesn't support a custom location for generating profile
    // information.
    //
    // llvm_out/build/profiles/ is the location this writes to.
    /// generate PGO profile with llvm built for rustc
    #[arg(global = true, long)]
    pub llvm_profile_generate: bool,
    /// Enable BOLT link flags
    #[arg(global = true, long)]
    pub enable_bolt_settings: bool,
    /// Skip stage0 compiler validation
    #[arg(global = true, long)]
    pub skip_stage0_validation: bool,
    /// Additional reproducible artifacts that should be added to the reproducible artifacts archive.
    #[arg(global = true, long)]
    pub reproducible_artifact: Vec<String>,
    #[arg(global = true)]
    /// paths for the subcommand
    pub paths: Vec<PathBuf>,
    /// override options in config.toml
    #[arg(global = true, value_hint = clap::ValueHint::Other, long, value_name = "section.option=value")]
    pub set: Vec<String>,
    /// arguments passed to subcommands
    #[arg(global = true, last(true), value_name = "ARGS")]
    pub free_args: Vec<String>,
}

impl Flags {
    /// Check if `<cmd> -h -v` was passed.
    /// If yes, print the available paths and return `true`.
    pub fn try_parse_verbose_help(args: &[String]) -> bool {
        // We need to check for `<cmd> -h -v`, in which case we list the paths
        #[derive(Parser)]
        #[command(disable_help_flag(true))]
        struct HelpVerboseOnly {
            #[arg(short, long)]
            help: bool,
            #[arg(global = true, short, long, action = clap::ArgAction::Count)]
            pub verbose: u8,
            #[arg(value_enum)]
            cmd: Kind,
        }
        if let Ok(HelpVerboseOnly { help: true, verbose: 1.., cmd: subcommand }) =
            HelpVerboseOnly::try_parse_from(normalize_args(args))
        {
            println!("NOTE: updating submodules before printing available paths");
            let config = Config::parse(Self::parse(&[String::from("build")]));
            let build = Build::new(config);
            let paths = Builder::get_help(&build, subcommand);
            if let Some(s) = paths {
                println!("{s}");
            } else {
                panic!("No paths available for subcommand `{}`", subcommand.as_str());
            }
            true
        } else {
            false
        }
    }

    pub fn parse(args: &[String]) -> Self {
        Flags::parse_from(normalize_args(args))
    }
}

pub fn normalize_args(args: &[String]) -> Vec<String> {
    let first = String::from("x.py");
    let it = std::iter::once(first).chain(args.iter().cloned());
    it.collect()
}

