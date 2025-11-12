use build_helper::prelude::*;
use build_helper::exit;
use crate::Kind;
use crate::DocTests;
#[derive(Debug, Clone, clap::Subcommand)]
pub enum QaTool {
    Bench {
        #[arg(long)]
        /// Run `rustc-bench-output` tests
        rustc_bench_output: bool,
    },
    Check {
        #[arg(long)]
        /// Run `rustc` tests
        rustc: bool,
        #[arg(long)]
        /// Run `rustdoc` tests
        rustdoc: bool,
        #[arg(long)]
        /// Run `rustfmt` tests
        rustfmt: bool,
        #[arg(long)]
        /// Run `clippy` tests
        clippy: bool,
        #[arg(long)]
        /// Run `miri` tests
        miri: bool,
        #[arg(long)]
        /// Run `cargo` tests
        cargo: bool,
        #[arg(long)]
        /// Run `rust-analyzer` tests
        rust_analyzer: bool,
        #[arg(long)]
        /// Run `tidy` tests
        tidy: bool,
        #[arg(long)]
        /// Run `html` tests
        html: bool,
        #[arg(long)]
        /// Run `linkcheck` tests
        linkcheck: bool,
        #[arg(long)]
        /// Run `error-index` tests
        error_index: bool,
        #[arg(long)]
        /// Run `bootstrap` tests
        bootstrap: bool,
        #[arg(long)]
        /// Run `build-helper` tests
        build_helper: bool,
        #[arg(long)]
        /// Run `rustdoc-json-types` tests
        rustdoc_json_types: bool,
        #[arg(long)]
        /// Run `rustdoc-gui` tests
        rustdoc_gui: bool,
        #[arg(long)]
        /// Run `rustdoc-js-std` tests
        rustdoc_js_std: bool,
        #[arg(long)]
        /// Run `rustdoc-js-not-std` tests
        rustdoc_js_not_std: bool,
        #[arg(long)]
        /// Run `rustdoc-theme` tests
        rustdoc_theme: bool,
        #[arg(long)]
        /// Run `test-float-parse` tests
        test_float_parse: bool,
        #[arg(long)]
        /// Run `compiletest` tests
        compiletest: bool,
        #[arg(long)]
        /// Run `run-make-support` tests
        run_make_support: bool,
        #[arg(long)]
        /// Run `rustc-guide` tests
        rustc_guide: bool,
        #[arg(long)]
        /// Run `embedded-book` tests
        embedded_book: bool,
        #[arg(long)]
        /// Run `edition-guide` tests
        edition_guide: bool,
        #[arg(long)]
        /// Run `nomicon` tests
        nomicon: bool,
        #[arg(long)]
        /// Run `reference` tests
        reference: bool,
        #[arg(long)]
        /// Run `rustdoc-book` tests
        rustdoc_book: bool,
        #[arg(long)]
        /// Run `rust-by-example` tests
        rust_by_example: bool,
        #[arg(long)]
        /// Run `the-book` tests
        the_book: bool,
        #[arg(long)]
        /// Run `unstable-book` tests
        unstable_book: bool,
        #[arg(long)]
        /// Run `miri-test` tests
        miri_test: bool,
        #[arg(long)]
        /// Run `clippy-test` tests
        clippy_test: bool,
        #[arg(long)]
        /// Run `rustfmt-test` tests
        rustfmt_test: bool,
        #[arg(long)]
        /// Run `bootstrap-test` tests
        bootstrap_test: bool,
        #[arg(long)]
        /// Run `build-helper-test` tests
        build_helper_test: bool,
        #[arg(long)]
        /// Run `rustdoc-json-types-test` tests
        rustdoc_json_types_test: bool,
        #[arg(long)]
        /// Run `rustdoc-gui-test` tests
        rustdoc_gui_test: bool,
        #[arg(long)]
        /// Run `rustdoc-js-std-test` tests
        rustdoc_js_std_test: bool,
        #[arg(long)]
        /// Run `rustdoc-js-not-std-test` tests
        rustdoc_js_not_std_test: bool,
        #[arg(long)]
        /// Run `rustdoc-theme-test` tests
        rustdoc_theme_test: bool,
        #[arg(long)]
        /// Run `test-float-parse-test` tests
        test_float_parse_test: bool,
        #[arg(long)]
        /// Run `compiletest-test` tests
        compiletest_test: bool,
        #[arg(long)]
        /// Run `run-make-support-test` tests
        run_make_support_test: bool,
        #[arg(long)]
        /// Run `rustc-guide-test` tests
        rustc_guide_test: bool,
        #[arg(long)]
        /// Run `embedded-book-test` tests
        embedded_book_test: bool,
        #[arg(long)]
        /// Run `edition-guide-test` tests
        edition_guide_test: bool,
        #[arg(long)]
        /// Run `nomicon-test` tests
        nomicon_test: bool,
        #[arg(long)]
        /// Run `reference-test` tests
        reference_test: bool,
        #[arg(long)]
        /// Run `rustdoc-book-test` tests
        rustdoc_book_test: bool,
        #[arg(long)]
        /// Run `rust-by-example-test` tests
        rust_by_example_test: bool,
        #[arg(long)]
        /// Run `the-book-test` tests
        the_book_test: bool,
        #[arg(long)]
        /// Run `unstable-book-test` tests
        unstable_book_test: bool,
    },
    Clippy {
        #[arg(long)]
        /// Run `clippy` tests
        clippy: bool,
    },
    Fix {
        #[arg(long)]
        /// Run `fix` tests
        fix: bool,
    },
    Format {
        #[arg(long)]
        /// Run `format` tests
        format: bool,
    },
    Test {
        #[arg(long)]
        /// Run `test` tests
        test: bool,
        #[arg(long)]
        /// Run `doc` tests
        doc: bool,
        #[arg(long)]
        /// Do not run `doc` tests
        no_doc: bool,
        #[arg(long)]
        /// Do not fail fast
        no_fail_fast: bool,
        #[arg(long)]
        /// Bless all tests
        bless: bool,
        #[arg(long)]
        /// Extra checks
        extra_checks: Option<String>,
        #[arg(long)]
        /// Only modified tests
        only_modified: bool,
        #[arg(long)]
        /// Force rerun
        force_rerun: bool,
        #[arg(long)]
        /// Rustfix coverage
        rustfix_coverage: bool,
        #[arg(long)]
        /// Compare mode
        compare_mode: Option<String>,
        #[arg(long)]
        /// Pass
        pass: Option<String>,
        #[arg(long)]
        /// Run
        run: Option<String>,
        #[arg(long)]
        /// Compiletest rustc args
        compiletest_rustc_args: Vec<String>,
    },
    Miri {
        #[arg(long)]
        /// Run `miri` tests
        miri: bool,
        #[arg(long)]
        /// Do not run `doc` tests
        no_doc: bool,
        #[arg(long)]
        /// Run `doc` tests
        doc: bool,
        #[arg(long)]
        /// Do not fail fast
        no_fail_fast: bool,
    },
    Suggest {
        #[arg(long)]
        /// Run `suggest` tests
        suggest: bool,
    },
    Perf {
        #[arg(long)]
        /// Run `perf` tests
        perf: bool,
    },
}
#[derive(Debug, Clone, clap::Subcommand)]
pub enum BuildTool {
    Build {
        #[arg(long)]
        /// Run `build` tests
        build: bool,
    },
    Doc {
        #[arg(long)]
        /// Open
        open: bool,
        #[arg(long)]
        /// Json
        json: bool,
    },
}
#[derive(Debug, Clone, clap::Subcommand)]
pub enum DistTool {
    Dist {
        #[arg(long)]
        /// Run `dist` tests
        dist: bool,
    },
    Install {
        #[arg(long)]
        /// Run `install` tests
        install: bool,
    },
}
#[derive(Debug, Clone, clap::Subcommand)]
pub enum MiscTool {
    Clean {
        #[arg(long)]
        /// Run `clean` tests
        clean: bool,
    },
    Run {
        #[arg(long)]
        /// Run `run` tests
        run: bool,
    },
    Setup {
        #[arg(long)]
        /// Run `setup` tests
        setup: bool,
    },
    Vendor {
        #[arg(long)]
        /// Run `vendor` tests
        vendor: bool,
        #[arg(long)]
        /// Versioned dirs
        versioned_dirs: bool,
        #[arg(long)]
        /// Sync
        sync: Vec<PathBuf>,
    },
}
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
            Subcommand::Qa(qa_tool) => {
                match qa_tool {
                    QaTool::Bench { .. } => Kind::Bench,
                    QaTool::Check { .. } => Kind::Check,
                    QaTool::Clippy { .. } => Kind::Clippy,
                    QaTool::Fix { .. } => Kind::Fix,
                    QaTool::Format { .. } => Kind::Format,
                    QaTool::Test { .. } => Kind::Test,
                    QaTool::Miri { .. } => Kind::Miri,
                    QaTool::Suggest { .. } => Kind::Suggest,
                    QaTool::Perf { .. } => Kind::Perf,
                }
            }
            Subcommand::Build(build_tool) => {
                match build_tool {
                    BuildTool::Build { .. } => Kind::Build,
                    BuildTool::Doc { .. } => Kind::Doc,
                }
            }
            Subcommand::Dist(dist_tool) => {
                match dist_tool {
                    DistTool::Dist { .. } => Kind::Dist,
                    DistTool::Install { .. } => Kind::Install,
                }
            }
            Subcommand::Misc(misc_tool) => {
                match misc_tool {
                    MiscTool::Clean { .. } => Kind::Clean,
                    MiscTool::Run { .. } => Kind::Run,
                    MiscTool::Setup { .. } => Kind::Setup,
                    MiscTool::Vendor { .. } => Kind::Vendor,
                }
            }
        }
    }
    pub fn compiletest_rustc_args(&self) -> Vec<&str> {
        match self {
            Subcommand::Qa(QaTool::Test { ref compiletest_rustc_args, .. }) => {
                compiletest_rustc_args
                    .iter()
                    .flat_map(|s| s.split_whitespace())
                    .collect()
            }
            _ => vec![],
        }
    }
    pub fn fail_fast(&self) -> bool {
        match self {
            Subcommand::Qa(QaTool::Test { no_fail_fast, .. })
            | Subcommand::Qa(QaTool::Miri { no_fail_fast, .. }) => !no_fail_fast,
            _ => false,
        }
    }
    pub fn doc_tests(&self) -> DocTests {
        match self {
            Subcommand::Qa(QaTool::Test { doc, no_doc, .. })
            | Subcommand::Qa(QaTool::Miri { no_doc, doc, .. }) => {
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
            Subcommand::Qa(QaTool::Test { ref extra_checks, .. }) => {
                extra_checks.as_ref().map(String::as_str)
            }
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
            Subcommand::Qa(QaTool::Test { ref compare_mode, .. }) => {
                compare_mode.as_ref().map(|s| &s[..])
            }
            _ => None,
        }
    }
    pub fn pass(&self) -> Option<&str> {
        match self {
            Subcommand::Qa(QaTool::Test { ref pass, .. }) => {
                pass.as_ref().map(|s| &s[..])
            }
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
use crate::Flags;
pub fn get_completion<G: clap_complete::Generator>(
    shell: G,
    path: &Path,
) -> Option<String> {
    let mut cmd = Flags::command();
    let current = if !path.exists() {
        String::new()
    } else {
        std::fs::read_to_string(path)
            .unwrap_or_else(|_| {
                eprintln!("couldn't read {}", path.display());
                exit!(1)
            })
    };
    let mut buf = Vec::new();
    clap_complete::generate(shell, &mut cmd, "x.py", &mut buf);
    if buf == current.as_bytes() {
        return None;
    }
    Some(String::from_utf8(buf).expect("completion script should be UTF-8"))
}
