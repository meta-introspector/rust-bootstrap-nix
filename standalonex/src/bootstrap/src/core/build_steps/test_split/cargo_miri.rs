use crate::prelude::*


//use crate::core::build_steps::common_test_fields;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CargoMiri {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for CargoMiri {
    type Output = ();
    const ONLY_HOSTS: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/tools/miri/cargo-miri")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.target);
        run.builder.ensure(CargoMiri {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.builder.build.build,
                compiler,
                target: run.target,
            },
        });
    }

    /// Tests `cargo miri test`.
    fn run(self, builder: &Builder<'_>) {
        let host = self.common.host;
        let target = self.common.target;
        let stage = self.common.stage;
        if stage == 0 {
            eprintln!("cargo-miri cannot be tested at stage 0");
            std::process::exit(1);
        }

        // This compiler runs on the host, we'll just use it for the target.
        let compiler = builder.compiler(stage, host);

        // Run `cargo miri test`.
        // This is just a smoke test (Miri's own CI invokes this in a bunch of different ways and ensures
        // that we get the desired output), but that is sufficient to make sure that the libtest harness
        // itself executes properly under Miri, and that all the logic in `cargo-miri` does not explode.
        let mut cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolStd, // it's unclear what to use here, we're not building anything just doing a smoke test!
            target,
            Kind::MiriTest,
            "src/tools/miri/test-cargo-miri",
            SourceType::Submodule,
            &[],
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
