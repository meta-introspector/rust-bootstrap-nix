use crate::prelude::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rustfmt {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for Rustfmt {
    type Output = ();
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/tools/rustfmt")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.target);
        run.builder.ensure(Rustfmt {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.target,
                compiler,
                target: run.target,
            },
        });
    }

    /// Runs `cargo test` for rustfmt.
    fn run(self, builder: &Builder<'_>) {
        let stage = self.common.stage;
        let host = self.common.host;
        let compiler = self.common.compiler;

        builder.ensure(tool::Rustfmt { compiler, target: self.common.host, extra_features: Vec::new() });

        let mut cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolRustc,
            host,
            Kind::Test,
            "src/tools/rustfmt",
            SourceType::InTree,
            &[],
        );

        let dir = testdir(builder, compiler.host);
        t!(fs::create_dir_all(&dir));
        cargo.env("RUSTFMT_TEST_DIR", dir);

        cargo.add_rustc_lib_path(builder);

        run_cargo_test(cargo, &[], &[], "rustfmt", "rustfmt", compiler, host, builder);
    }
}
