#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateBuildHelper {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for CrateBuildHelper {
    type Output = ();
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/build_helper")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.target);
        run.builder.ensure(CrateBuildHelper {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.target,
                compiler,
                target: run.target,
            },
        });
    }

    /// Runs `cargo test` for build_helper.
    fn run(self, builder: &Builder<'_>) {
        let host = self.common.host;
        let compiler = self.common.compiler;

        let mut cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolBootstrap,
            host,
            Kind::Test,
            "src/build_helper",
            SourceType::InTree,
            &[],
        );
        cargo.allow_features("test");
        run_cargo_test(
            cargo,
            &[],
            &[],
            "build_helper",
            "build_helper self test",
            compiler,
            host,
            builder,
        );
    }
}
