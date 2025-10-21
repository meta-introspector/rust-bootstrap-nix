#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RustdocJSStd {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for RustdocJSStd {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let default = run.builder.config.nodejs.is_some();
        run.suite_path("tests/rustdoc-js-std").default_condition(default)
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.target);
        run.builder.ensure(RustdocJSStd {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.target,
                compiler,
                target: run.target,
            },
        });
    }

    fn run(self, builder: &Builder<'_>) {
        let nodejs =
            builder.config.nodejs.as_ref().expect("need nodejs to run rustdoc-js-std tests");
        let mut command = command(nodejs);
        command
            .arg(builder.src.join("src/tools/rustdoc-js/tester.js"))
            .arg("--crate-name")
            .arg("std")
            .arg("--resource-suffix")
            .arg(&builder.version)
            .arg("--doc-folder")
            .arg(builder.doc_out(self.common.target))
            .arg("--test-folder")
            .arg(builder.src.join("tests/rustdoc-js-std"));
        for path in &builder.paths {
            if let Some(p) = helpers::is_valid_test_suite_arg(path, "tests/rustdoc-js-std", builder)
            {
                if !p.ends_with(".js") {
                    eprintln!("A non-js file was given: `{}`", path.display());
                    panic!("Cannot run rustdoc-js-std tests");
                }
                command.arg("--test-file").arg(path);
            }
        }
        builder.ensure(crate::core::build_steps::doc::Std::new(
            self.common.stage,
            self.common.target,
            DocumentationFormat::Html,
        ));
        let _guard = builder.msg(
            Kind::Test,
            self.common.stage,
            "rustdoc-js-std",
            builder.config.build,
            self.common.target,
        );
        command.run(builder);
    }
}
