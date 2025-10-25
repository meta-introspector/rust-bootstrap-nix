        );

        // We're not using `prepare_cargo_test` so we have to do this ourselves.
        // (We're not using that as the test-cargo-miri crate is not known to bootstrap.)
        match builder.doc_tests {
            DocTests::Yes => {}
            DocTests::No => {
                cargo.args(["--lib", "--bins", "--examples", "--tests", "--benches"]);
            }
            DocTests::Only => {
                cargo.arg("--doc");
            }
        }

        // Finally, pass test-args and run everything.
        cargo.arg("--").args(builder.config.test_args());
        let mut cargo = BootstrapCommand::from(cargo);
        {
            let _guard = builder.msg_sysroot_tool(Kind::Test, stage, "cargo-miri", host, target);
            let _time = helpers::timeit(builder);
            cargo.run(builder);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompiletestTest {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for CompiletestTest {
    type Output = ();

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/tools/compiletest")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.target);
        run.builder.ensure(CompiletestTest {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.target,
                compiler,
                target: run.target,
            },
        });
    }

    /// Runs `cargo test` for compiletest.
    fn run(self, builder: &Builder<'_>) {
        let host = self.common.host;
        let compiler = self.common.compiler;

        // We need `ToolStd` for the locally-built sysroot because
        // compiletest uses unstable features of the `test` crate.
        builder.ensure(compile::Std::new(compiler, host));
        let mut cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            // compiletest uses libtest internals; make it use the in-tree std to make sure it never breaks
            // when std sources change.
            Mode::ToolStd,
            host,
            Kind::Test,
            "src/tools/compiletest",
            SourceType::InTree,
            &[],
        );
        cargo.allow_features("test");
        run_cargo_test(
            cargo,
            &[],
            &[],
            "compiletest",
            "compiletest self test",
