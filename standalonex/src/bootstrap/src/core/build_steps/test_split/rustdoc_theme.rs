#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RustdocTheme {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for RustdocTheme {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/tools/rustdoc-themes")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.target);

        run.builder.ensure(RustdocTheme {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.target,
                compiler,
                target: run.target,
            },
        });
    }

    fn run(self, builder: &Builder<'_>) {
        let rustdoc = builder.bootstrap_out.join("rustdoc");
        let mut cmd = builder.tool_cmd(Tool::RustdocTheme);
        cmd.arg(rustdoc.to_str().unwrap())
            .arg(builder.src.join("src/librustdoc/html/static/css/rustdoc.css").to_str().unwrap())
            .env("RUSTC_STAGE", self.common.compiler.stage.to_string())
            .env("RUSTC_SYSROOT", builder.sysroot(self.common.compiler))
            .env("RUSTDOC_LIBDIR", builder.sysroot_target_libdir(self.common.compiler, self.common.compiler.host))
            .env("CFG_RELEASE_CHANNEL", &builder.config.channel)
            .env("RUSTDOC_REAL", builder.rustdoc(self.common.compiler))
            .env("RUSTC_BOOTSTRAP", "1");
        cmd.args(linker_args(builder, self.common.compiler.host, LldThreads::No));

        cmd.delay_failure().run(builder);
    }
}
