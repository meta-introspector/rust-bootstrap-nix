use crate::prelude::*;


    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/tools/cargotest")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.target);
        run.builder.ensure(Cargotest {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.target,
                compiler,
                target: run.target,
            },
        });
    }

    /// Runs the `cargotest` tool as compiled in `stage` by the `host` compiler.
    ///
    /// This tool in `src/tools` will check out a few Rust projects and run `cargo
    /// test` to ensure that we don't regress the test suites there.
    fn run(self, builder: &Builder<'_>) {
        let compiler = self.common.compiler;
        let host = self.common.host;
        builder.ensure(compile::Rustc::new(compiler, compiler.host));
        let cargo = builder.ensure(tool::Cargo { compiler, target: compiler.host });

        // Note that this is a short, cryptic, and not scoped directory name. This
        // is currently to minimize the length of path on Windows where we otherwise
        // quickly run into path name limit constraints.
        let out_dir = builder.out.join("ct");
        t!(fs::create_dir_all(&out_dir));

        let _time = helpers::timeit(builder);
        let mut cmd = builder.tool_cmd(Tool::CargoTest);
        cmd.arg(&cargo)
            .arg(&out_dir)
            .args(builder.config.test_args())
            .env("RUSTC", builder.rustc(compiler))
            .env("RUSTDOC", builder.rustdoc(compiler));
        add_rustdoc_cargo_linker_args(&mut cmd, builder, compiler.host, LldThreads::No);
        cmd.delay_failure().run(builder);
    }
