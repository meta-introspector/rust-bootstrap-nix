use crate::prelude::*


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RustdocJSNotStd {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for RustdocJSNotStd {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let default = run.builder.config.nodejs.is_some();
        run.suite_path("tests/rustdoc-js").default_condition(default)
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.build_triple());
        run.builder.ensure(RustdocJSNotStd {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.build_triple(),
                compiler,
                target: run.target,
            },
        });
    }

    fn run(self, builder: &Builder<'_>) {
        builder.ensure(Compiletest {
            compiler: self.common.compiler,
            target: self.common.target,
            mode: "js-doc-test",
            suite: "rustdoc-js",
            path: "tests/rustdoc-js",
            compare_mode: None,
        });
    }
}
