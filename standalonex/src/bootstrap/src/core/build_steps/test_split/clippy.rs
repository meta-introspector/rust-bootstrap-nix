use crate::prelude::*


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Clippy {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for Clippy {
    type Output = ();
    const ONLY_HOSTS: bool = true;
    const DEFAULT: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/tools/clippy")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.target);
        run.builder.ensure(Clippy {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.target,
                compiler,
                target: run.target,
            },
        });
    }

    /// Runs `cargo test` for clippy.
    fn run(self, builder: &Builder<'_>) {
        let stage = self.common.stage;
        let host = self.common.host;
        let compiler = self.common.compiler;

        builder.ensure(tool::Clippy { compiler, target: self.common.host, extra_features: Vec::new() });
        let mut cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolRustc,
            host,
            Kind::Test,
            "src/tools/clippy",
            SourceType::InTree,
            &[],
        );

        cargo.env("RUSTC_TEST_SUITE", builder.rustc(compiler));
        cargo.env("RUSTC_LIB_PATH", builder.rustc_libdir(compiler));
        let host_libs = builder.stage_out(compiler, Mode::ToolRustc).join(builder.cargo_dir());
        cargo.env("HOST_LIBS", host_libs);

        cargo.add_rustc_lib_path(builder);
        let cargo = prepare_cargo_test(cargo, &[], &[], "clippy", compiler, host, builder);

        let _guard = builder.msg_sysroot_tool(Kind::Test, compiler.stage, "clippy", host, host);

        // Clippy reports errors if it blessed the outputs
        if cargo.allow_failure().run(builder) {
            // The tests succeeded; nothing to do.
            return;
        }

        if !builder.config.cmd.bless() {
            crate::exit!(1);
        }
    }
}
