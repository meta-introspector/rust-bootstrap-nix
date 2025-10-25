use crate::prelude::*


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LintDocs {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for LintDocs {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/tools/lint-docs")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.builder.config.build);
        run.builder.ensure(LintDocs {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.builder.config.build,
                compiler,
                target: run.target,
            },
        });
    }

    /// Tests that the lint examples in the rustc book generate the correct
    /// lints and have the expected format.
    fn run(self, builder: &Builder<'_>) {
        builder.ensure(crate::core::build_steps::doc::RustcBook {
            compiler: self.common.compiler,
            target: self.common.target,
            validate: true,
        });
    }
}
