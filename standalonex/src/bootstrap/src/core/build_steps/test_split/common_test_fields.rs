use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommonTestFields {
    pub compiler: Compiler,
    pub target: TargetSelection,
    pub host: TargetSelection,
    pub doc_tests: DocTests,
    pub no_fail_fast: bool,
    pub bless: bool,
    pub extra_checks: Option<String>,
    pub only_modified: bool,
    pub force_rerun: bool,
    pub rustfix_coverage: bool,
    pub compare_mode: Option<String>,
    pub pass: Option<String>,
    pub run: Option<String>,
    pub compiletest_rustc_args: Vec<String>,
}

impl CommonTestFields {
    pub fn new(builder: &Builder<'_>, compiler: Compiler, target: TargetSelection) -> Self {
        let subcommand = builder.config.cmd.clone();
        let (doc_tests, no_fail_fast, bless, extra_checks, only_modified, force_rerun, rustfix_coverage, compare_mode, pass, run, compiletest_rustc_args) = match subcommand {
            Subcommand::Qa(QaTool::Test { doc, no_doc, no_fail_fast, bless, extra_checks, only_modified, force_rerun, rustfix_coverage, compare_mode, pass, run, compiletest_rustc_args }) => {
                let doc_tests = if doc {
                    DocTests::Only
                } else if no_doc {
                    DocTests::No
                } else {
                    DocTests::Yes
                };
                (doc_tests, no_fail_fast, bless, extra_checks, only_modified, force_rerun, rustfix_coverage, compare_mode, pass, run, compiletest_rustc_args)
            }
            _ => (DocTests::Yes, false, false, None, false, false, false, None, None, None, vec![]),
        };

        Self {
            compiler,
            target,
            host: builder.config.build,
            doc_tests,
            no_fail_fast,
            bless,
            extra_checks,
            only_modified,
            force_rerun,
            rustfix_coverage,
            compare_mode,
            pass,
            run,
            compiletest_rustc_args,
        }
    }
}
