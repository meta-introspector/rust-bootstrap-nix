use crate::prelude::*


    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Miri {
    pub common: common_test_fields::CommonTestFields,
}

impl Miri {
    /// Run `cargo miri setup` for the given target, return where the Miri sysroot was put.
    pub fn build_miri_sysroot(
        builder: &Builder<'_>,
        compiler: Compiler,
        target: TargetSelection,
    ) -> PathBuf {
        let miri_sysroot = builder.out.join(compiler.host).join("miri-sysroot");
        let mut cargo = builder::Cargo::new(
            builder,
            compiler,
            Mode::Std,
            SourceType::Submodule,
            target,
            Kind::MiriSetup,
        );

        // Tell `cargo miri setup` where to find the sources.
        cargo.env("MIRI_LIB_SRC", builder.src.join("library"));
        // Tell it where to put the sysroot.
        cargo.env("MIRI_SYSROOT", &miri_sysroot);

        let mut cargo = BootstrapCommand::from(cargo);
        let _guard =
            builder.msg(Kind::Build, compiler.stage, "miri sysroot", compiler.host, target);
        cargo.run(builder);

        // # Determine where Miri put its sysroot.
        // To this end, we run `cargo miri setup --print-sysroot` and capture the output.
        // (We do this separately from the above so that when the setup actually
        // happens we get some output.)
        // We re-use the `cargo` from above.
        cargo.arg("--print-sysroot");

        builder.verbose(|| println!("running: {cargo:?}"));
        let stdout = cargo.run_capture_stdout(builder).stdout();
        // Output is "<sysroot>\n".
        let sysroot = stdout.trim_end();
        builder.verbose(|| println!("`cargo miri setup --print-sysroot` said: {sysroot:?}"));
        PathBuf::from(sysroot)
    }
}

impl Step for Miri {
    type Output = ();
    const ONLY_HOSTS: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/tools/miri")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.target);
        run.builder.ensure(Miri {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.builder.build.build,
                compiler,
                target: run.target,
            },
        });
    }

    /// Runs `cargo test` for miri.
    fn run(self, builder: &Builder<'_>) {
        let host = self.common.host;
        let target = self.common.target;
        let stage = self.common.stage;
        if stage == 0 {
            eprintln!("miri cannot be tested at stage 0");
            std::process::exit(1);
        }

        // This compiler runs on the host, we'll just use it for the target.
        let target_compiler = builder.compiler(stage, host);
        // Similar to `compile::Assemble`, build with the previous stage's compiler. Otherwise
        // we'd have stageN/bin/rustc and stageN/bin/rustdoc be effectively different stage
        // compilers, which isn't what we want. Rustdoc should be linked in the same way as the
        // rustc compiler it's paired with, so it must be built with the previous stage compiler.
        let host_compiler = builder.compiler(stage - 1, host);

        // Build our tools.
        let miri = builder.ensure(tool::Miri {
            compiler: host_compiler,
            target: host,
            extra_features: Vec::new(),
        });
        // the ui tests also assume cargo-miri has been built
        builder.ensure(tool::CargoMiri {
