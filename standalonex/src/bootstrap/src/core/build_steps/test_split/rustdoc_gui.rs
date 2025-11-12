#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RustdocGUI {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for RustdocGUI {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let builder = run.builder;
        let run = run.suite_path("tests/rustdoc-gui");
        run.lazy_default_condition(Box::new(move || {
            builder.config.nodejs.is_some()
                && builder.doc_tests != DocTests::Only
                && builder
                    .config
                    .npm
                    .as_ref()
                    .map(|p| get_browser_ui_test_version(builder, p).is_some())
                    .unwrap_or(false)
        }))
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.build_triple());
        run.builder.ensure(RustdocGUI {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.build_triple(),
                compiler,
                target: run.target,
            },
        });
    }

    fn run(self, builder: &Builder<'_>) {
        builder.ensure(compile::Std::new(self.common.compiler, self.common.target));

        let mut cmd = builder.tool_cmd(Tool::RustdocGUITest);

        let out_dir = builder.test_out(self.common.target).join("rustdoc-gui");
        builder.clear_if_dirty(&out_dir, &builder.rustdoc(self.common.compiler));

        if let Some(src) = builder.config.src.to_str() {
            cmd.arg("--rust-src").arg(src);
        }

        if let Some(out_dir) = out_dir.to_str() {
            cmd.arg("--out-dir").arg(out_dir);
        }

        if let Some(initial_cargo) = builder.config.initial_cargo.to_str() {
            cmd.arg("--initial-cargo").arg(initial_cargo);
        }

        cmd.arg("--jobs").arg(builder.jobs().to_string());

        cmd.env("RUSTDOC", builder.rustdoc(self.common.compiler))
            .env("RUSTC", builder.rustc(self.common.compiler));

        add_rustdoc_cargo_linker_args(&mut cmd, builder, self.common.compiler.host, LldThreads::No);

        for path in &builder.paths {
            if let Some(p) = helpers::is_valid_test_suite_arg(p, "tests/rustdoc-gui", builder) {
                if !p.ends_with(".goml") {
                    eprintln!("A non-goml file was given: `{}`", path.display());
                    panic!("Cannot run rustdoc-gui tests");
                }
                if let Some(name) = path.file_name().and_then(|f| f.to_str()) {
                    cmd.arg("--goml-file").arg(name);
                }
            }
        }

        for test_arg in builder.config.test_args() {
            cmd.arg("--test-arg").arg(test_arg);
        }

        if let Some(ref nodejs) = builder.config.nodejs {
            cmd.arg("--nodejs").arg(nodejs);
        }

        if let Some(ref npm) = builder.config.npm {
            cmd.arg("--npm").arg(npm);
        }

        let _time = helpers::timeit(builder);
        let _guard = builder.msg_sysroot_tool(
            Kind::Test,
            self.common.compiler.stage,
            "rustdoc-gui",
            self.common.compiler.host,
            self.common.target,
        );
        try_run_tests(builder, &mut cmd, true);
    }
}
