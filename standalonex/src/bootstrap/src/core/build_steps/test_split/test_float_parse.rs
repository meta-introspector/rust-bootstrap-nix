use crate::prelude::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TestFloatParse {
    pub common: common_test_fields::CommonTestFields,
    path: PathBuf,
}

impl Step for TestFloatParse {
    type Output = ();
    const ONLY_HOSTS: bool = true;
    const DEFAULT: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/etc/test-float-parse")
    }

    fn make_run(run: RunConfig<'_>) {
        for path in run.paths {
            let path = path.assert_single_path().path.clone();
            let compiler = run.builder.compiler(run.builder.top_stage, run.target);
            run.builder.ensure(Self {
                path,
                common: common_test_fields::CommonTestFields {
                    stage: run.builder.top_stage,
                    host: run.target,
                    compiler,
                    target: run.target,
                },
            });
        }
    }

    fn run(self, builder: &Builder<'_>) {
        let bootstrap_host = self.common.host;
        let compiler = self.common.compiler;
        let path = self.path.to_str().unwrap();
        let crate_name = self.path
            .components()
            .last()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap();

        builder.ensure(tool::TestFloatParse { host: self.common.host });

        // Run any unit tests in the crate
        let cargo_test = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolStd,
            bootstrap_host,
            Kind::Test,
            path,
            SourceType::InTree,
            &[],
        );

        run_cargo_test(
            cargo_test,
            &[],
            &[],
            crate_name,
            crate_name,
            compiler,
            bootstrap_host,
            builder,
        );

        // Run the actual parse tests.
        let mut cargo_run = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolStd,
            bootstrap_host,
            Kind::Run,
            path,
            SourceType::InTree,
            &[],
        );

        cargo_run.arg("--");
        if builder.config.args().is_empty() {
            // By default, exclude tests that take longer than ~1m.
            cargo_run.arg("--skip-huge");
        } else {
            cargo_run.args(builder.config.args());
        }

        cargo_run.into_cmd().run(builder);
    }
}
