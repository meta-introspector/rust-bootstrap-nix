use crate::prelude::*


                target: run.target,
            },
        });
    }

    fn run(self, builder: &Builder<'_>) {
        let compiler = self.common.compiler;
        let target = self.common.target;

        builder.ensure(compile::Std::new_with_extra_rust_args(compiler, target, &[
            "-Csymbol-mangling-version=v0",
            "-Cpanic=abort",
        ]));

        // If we're not doing a full bootstrap but we're testing a stage2
        // version of libstd, then what we're actually testing is the libstd
        // produced in stage1. Reflect that here by updating the compiler that
        // we're working with automatically.
        let compiler = builder.compiler_for(compiler.stage, compiler.host, target);

        let build_cargo = || {
            let mut cargo = builder::Cargo::new(
                builder,
                compiler,
                Mode::Codegen, // Must be codegen to ensure dlopen on compiled dylibs works
                SourceType::InTree,
                target,
                Kind::Run,
            );

            cargo.current_dir(&builder.src.join("compiler/rustc_codegen_gcc"));
            cargo
                .arg("--manifest-path")
                .arg(builder.src.join("compiler/rustc_codegen_gcc/build_system/Cargo.toml"));
            compile::rustc_cargo_env(builder, &mut cargo, target, compiler.stage);

            // Avoid incremental cache issues when changing rustc
            cargo.env("CARGO_BUILD_INCREMENTAL", "false");
            cargo.rustflag("-Cpanic=abort");

            cargo
        };

        builder.info(&format!(
            "{} GCC stage{} ({} -> {})",
            Kind::Test.description(),
            compiler.stage,
            &compiler.host,
            target
        ));
        let _time = helpers::timeit(builder);

        // FIXME: Uncomment the `prepare` command below once vendoring is implemented.
        /*
        let mut prepare_cargo = build_cargo();
        prepare_cargo.arg("--").arg("prepare");
        #[allow(deprecated)]
        builder.config.try_run(&mut prepare_cargo.into()).unwrap();
        */

        let mut cargo = build_cargo();

        cargo
            .arg("--")
            .arg("test")
            .arg("--use-system-gcc")
            .arg("--use-backend")
            .arg("gcc")
            .arg("--out-dir")
            .arg(builder.stage_out(compiler, Mode::ToolRustc).join("cg_gcc"))
            .arg("--release")
            .arg("--mini-tests")
            .arg("--std-tests");
        cargo.args(builder.config.test_args());

        cargo.into_cmd().run(builder);
    }
}

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
