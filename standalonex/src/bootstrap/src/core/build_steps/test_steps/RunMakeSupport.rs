use crate::prelude::*


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateRunMakeSupport {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for CrateRunMakeSupport {
    type Output = ();
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/tools/run-make-support")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.target);
        run.builder.ensure(CrateRunMakeSupport {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.target,
                compiler,
                target: run.target,
            },
        });
    }

    /// Runs `cargo test` for run-make-support.
    fn run(self, builder: &Builder<'_>) {
        let host = self.common.host;
        let compiler = self.common.compiler;

        let mut cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolBootstrap,
            host,
            Kind::Test,
            "src/tools/run-make-support",
            SourceType::InTree,
            &[],
        );
        cargo.allow_features("test");
        run_cargo_test(
            cargo,
            &[],
            &[],
            "run-make-support",
            "run-make-support self test",
            compiler,
            host,
            builder,
        );
