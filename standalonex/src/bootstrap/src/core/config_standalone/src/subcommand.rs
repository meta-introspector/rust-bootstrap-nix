use crate::prelude::*;

pub mod subcommand_groups;
use subcommand_groups::{QaTool, BuildTool, DistTool, MiscTool};

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Subcommand {
    Qa(QaTool),
    Build(BuildTool),
    Dist(DistTool),
    Misc(MiscTool),
}

impl Subcommand {
    pub fn kind(&self) -> Kind {
        match self {
            Subcommand::Qa(qa_tool) => match qa_tool {
                QaTool::Bench { .. } => Kind::Bench,
                QaTool::Check { .. } => Kind::Check,
                QaTool::Clippy { .. } => Kind::Clippy,
                QaTool::Fix { .. } => Kind::Fix,
                QaTool::Format { .. } => Kind::Format,
                QaTool::Test { .. } => Kind::Test,
                QaTool::Miri { .. } => Kind::Miri,
                QaTool::Suggest { .. } => Kind::Suggest,
                QaTool::Perf { .. } => Kind::Perf,
            },
            Subcommand::Build(build_tool) => match build_tool {
                BuildTool::Build { .. } => Kind::Build,
                BuildTool::Doc { .. } => Kind::Doc,
            },
            Subcommand::Dist(dist_tool) => match dist_tool {
                DistTool::Dist { .. } => Kind::Dist,
                DistTool::Install { .. } => Kind::Install,
            },
            Subcommand::Misc(misc_tool) => match misc_tool {
                MiscTool::Clean { .. } => Kind::Clean,
                MiscTool::Run { .. } => Kind::Run,
                MiscTool::Setup { .. } => Kind::Setup,
                MiscTool::Vendor { .. } => Kind::Vendor,
            },
        }
    }

    pub fn compiletest_rustc_args(&self) -> Vec<&str> {
        match self {
            Subcommand::Qa(QaTool::Test { ref compiletest_rustc_args, .. }) => {
                compiletest_rustc_args.iter().flat_map(|s| s.split_whitespace()).collect()
            }
            _ => vec![],
        }
    }

    pub fn fail_fast(&self) -> bool {
        match self {
            Subcommand::Qa(QaTool::Test { no_fail_fast, .. }) | Subcommand::Qa(QaTool::Miri { no_fail_fast, .. }) => {
                !no_fail_fast
            }
            _ => false,
        }
    }

    pub fn doc_tests(&self) -> DocTests {
        match self {
            Subcommand::Qa(QaTool::Test { doc, no_doc, .. }) | Subcommand::Qa(QaTool::Miri { no_doc, doc, .. }) => {
                if *doc {
                    DocTests::Only
                } else if *no_doc {
                    DocTests::No
                } else {
                    DocTests::Yes
                }
            }
            _ => DocTests::Yes,
        }
    }

    pub fn bless(&self) -> bool {
        match self {
            Subcommand::Qa(QaTool::Test { bless, .. }) => *bless,
            _ => false,
        }
    }

    pub fn extra_checks(&self) -> Option<&str> {
        match self {
            Subcommand::Qa(QaTool::Test { ref extra_checks, .. }) => extra_checks.as_ref().map(String::as_str),
            _ => None,
        }
    }

    pub fn only_modified(&self) -> bool {
        match self {
            Subcommand::Qa(QaTool::Test { only_modified, .. }) => *only_modified,
            _ => false,
        }
    }

    pub fn force_rerun(&self) -> bool {
        match self {
            Subcommand::Qa(QaTool::Test { force_rerun, .. }) => *force_rerun,
            _ => false,
        }
    }

    pub fn rustfix_coverage(&self) -> bool {
        match self {
            Subcommand::Qa(QaTool::Test { rustfix_coverage, .. }) => *rustfix_coverage,
            _ => false,
        }
    }

    pub fn compare_mode(&self) -> Option<&str> {
        match self {
            Subcommand::Qa(QaTool::Test { ref compare_mode, .. }) => compare_mode.as_ref().map(|s| &s[..]),
            _ => None,
        }
    }

    pub fn pass(&self) -> Option<&str> {
        match self {
            Subcommand::Qa(QaTool::Test { ref pass, .. }) => pass.as_ref().map(|s| &s[..]),
            _ => None,
        }
    }

    pub fn run(&self) -> Option<&str> {
        match self {
            Subcommand::Qa(QaTool::Test { ref run, .. }) => run.as_ref().map(|s| &s[..]),
            _ => None,
        }
    }

    pub fn open(&self) -> bool {
        match self {
            Subcommand::Build(BuildTool::Doc { open, .. }) => *open,
            _ => false,
        }
    }

    pub fn json(&self) -> bool {
        match self {
            Subcommand::Build(BuildTool::Doc { json, .. }) => *json,
            _ => false,
        }
    }

    pub fn vendor_versioned_dirs(&self) -> bool {
        match self {
            Subcommand::Misc(MiscTool::Vendor { versioned_dirs, .. }) => *versioned_dirs,
            _ => false,
        }
    }

    pub fn vendor_sync_args(&self) -> Vec<PathBuf> {
        match self {
            Subcommand::Misc(MiscTool::Vendor { sync, .. }) => sync.clone(),
            _ => vec![],
        }
    }
}

/// Returns the shell completion for a given shell, if the result differs from the current
/// content of `path`. If `path` does not exist, always returns `Some`.
pub fn get_completion<G: clap_complete::Generator>(shell: G, path: &Path) -> Option<String> {
    let mut cmd = Flags::command();
    let current = if !path.exists() {
        String::new()
    } else {
        std::fs::read_to_string(path).unwrap_or_else(|_| {
            eprintln!("couldn't read {}", path.display());
            crate::exit!(1)
        })
    };
    let mut buf = Vec::new();
    clap_complete::generate(shell, &mut cmd, "x.py", &mut buf);
    if buf == current.as_bytes() {
        return None;
    }
    Some(String::from_utf8(buf).expect("completion script should be UTF-8"))
}