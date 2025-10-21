#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RunMake {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for RunMake {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.suite_path("tests/run-make")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.build_triple());
        run.builder.ensure(RunMakeSupport {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.build_triple(),
                compiler,
                target: run.build_triple(),
            },
        });
        run.builder.ensure(RunMake {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.build_triple(),
                compiler,
                target: run.target,
            },
        });
    }

    fn run(self, builder: &Builder<'_>) {
        builder.ensure(Compiletest {
            compiler: self.common.compiler,
            target: self.common.target,
            mode: "run-make",
            suite: "run-make",
            path: "tests/run-make",
            compare_mode: None,
        });
    }
}
