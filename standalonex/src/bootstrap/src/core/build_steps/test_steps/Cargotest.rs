}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cargo {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for Cargo {
    type Output = ();
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/tools/cargo")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.target);
        run.builder.ensure(Cargo {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.target,
                compiler,
                target: run.target,
            },
        });
    }

    /// Runs `cargo test` for `cargo` packaged with Rust.
    fn run(self, builder: &Builder<'_>) {
        let compiler = self.common.compiler;
        let host = self.common.host;

        builder.ensure(tool::Cargo { compiler, target: self.common.host });
        let cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolRustc,
            self.common.host,
            Kind::Test,
            "src/tools/cargo",
            SourceType::Submodule,
            &[],
        );

        // NOTE: can't use `run_cargo_test` because we need to overwrite `PATH`
        let mut cargo = prepare_cargo_test(cargo, &[], &[], "cargo", compiler, self.common.host, builder);

        // Don't run cross-compile tests, we may not have cross-compiled libstd libs
        // available.
        cargo.env("CFG_DISABLE_CROSS_TESTS", "1");
        // Forcibly disable tests using nightly features since any changes to
        // those features won't be able to land.
        cargo.env("CARGO_TEST_DISABLE_NIGHTLY", "1");
