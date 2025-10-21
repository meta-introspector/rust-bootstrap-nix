#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RustAnalyzer {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for RustAnalyzer {
    type Output = ();
    const ONLY_HOSTS: bool = true;
    const DEFAULT: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/tools/rust-analyzer")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.build_triple());
        run.builder.ensure(Self {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.target,
                compiler,
                target: run.target,
            },
        });
    }

    /// Runs `cargo test` for rust-analyzer
    fn run(self, builder: &Builder<'_>) {
        let stage = self.common.stage;
        let host = self.common.host;
        let compiler = self.common.compiler;

        // We don't need to build the whole Rust Analyzer for the proc-macro-srv test suite,
        // but we do need the standard library to be present.
        builder.ensure(compile::Rustc::new(compiler, host));

        let workspace_path = "src/tools/rust-analyzer";
        // until the whole RA test suite runs on `i686`, we only run
        // `proc-macro-srv` tests
        let crate_path = "src/tools/rust-analyzer/crates/proc-macro-srv";
        let mut cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolRustc,
            host,
            Kind::Test,
            crate_path,
            SourceType::InTree,
            &["in-rust-tree".to_owned()],
        );
        cargo.allow_features(tool::RustAnalyzer::ALLOW_FEATURES);

        let dir = builder.src.join(workspace_path);
        // needed by rust-analyzer to find its own text fixtures, cf.
        // https://github.com/rust-analyzer/expect-test/issues/33
        cargo.env("CARGO_WORKSPACE_DIR", &dir);

        // RA's test suite tries to write to the source directory, that can't
        // work in Rust CI
        cargo.env("SKIP_SLOW_TESTS", "1");

        cargo.add_rustc_lib_path(builder);
        run_cargo_test(cargo, &[], &[], "rust-analyzer", "rust-analyzer", compiler, host, builder);
    }
}
