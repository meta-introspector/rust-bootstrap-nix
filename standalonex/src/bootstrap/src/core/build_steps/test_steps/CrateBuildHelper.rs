use crate::prelude::*;


    }
}

default_test!(Ui { path: "tests/ui", mode: "ui", suite: "ui" });

default_test!(Crashes { path: "tests/crashes", mode: "crashes", suite: "crashes" });

default_test!(Codegen { path: "tests/codegen", mode: "codegen", suite: "codegen" });

default_test!(CodegenUnits {
    path: "tests/codegen-units",
    mode: "codegen-units",
    suite: "codegen-units"
});

default_test!(Incremental { path: "tests/incremental", mode: "incremental", suite: "incremental" });

default_test_with_compare_mode!(Debuginfo {
    path: "tests/debuginfo",
    mode: "debuginfo",
    suite: "debuginfo",
    compare_mode: "split-dwarf"
});

host_test!(UiFullDeps { path: "tests/ui-fulldeps", mode: "ui", suite: "ui-fulldeps" });

host_test!(Rustdoc { path: "tests/rustdoc", mode: "rustdoc", suite: "rustdoc" });
host_test!(RustdocUi { path: "tests/rustdoc-ui", mode: "ui", suite: "rustdoc-ui" });

host_test!(RustdocJson { path: "tests/rustdoc-json", mode: "rustdoc-json", suite: "rustdoc-json" });

host_test!(Pretty { path: "tests/pretty", mode: "pretty", suite: "pretty" });

/// Special-handling is needed for `run-make`, so don't use `default_test` for defining `RunMake`
/// tests.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RunMake {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for RunMake {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.suite_path("tests/run-make")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.build_triple());
        run.builder.ensure(RunMakeSupport {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.build_triple(),
                compiler,
                target: run.build_triple(),
            },
        });
        run.builder.ensure(RunMake {
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
            mode: "run-make",
            suite: "run-make",
            path: "tests/run-make",
            compare_mode: None,
        });
    }
}

default_test!(Assembly { path: "tests/assembly", mode: "assembly", suite: "assembly" });

/// Coverage tests are a bit more complicated than other test suites, because
/// we want to run the same set of test files in multiple different modes,
/// in a way that's convenient and flexible when invoked manually.
