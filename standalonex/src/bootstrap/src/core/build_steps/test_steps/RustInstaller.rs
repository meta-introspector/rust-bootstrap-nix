    }

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/tools/rust-installer")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Self);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TestHelpers {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for TestHelpers {
    type Output = ();

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("tests/auxiliary/rust_test_helpers.c")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.target);
        run.builder.ensure(TestHelpers {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.target,
                compiler,
                target: run.target,
            },
        });
    }

    /// Compiles the `rust_test_helpers.c` library which we used in various
    /// `run-pass` tests for ABI testing.
    fn run(self, builder: &Builder<'_>) {
        if builder.config.dry_run {
            return;
        }
        // The x86_64-fortanix-unknown-sgx target doesn't have a working C
        // toolchain. However, some x86_64 ELF objects can be linked
        // without issues. Use this hack to compile the test helpers.
        let target = if self.common.target == "x86_64-fortanix-unknown-sgx" {
            TargetSelection::from_user("x86_64-unknown-linux-gnu")
        } else {
            self.common.target
        };
        let dst = builder.test_helpers_out(target);
        let src = builder.src.join("tests/auxiliary/rust_test_helpers.c");
        if up_to_date(&src, &dst.join("librust_test_helpers.a")) {
            return;
        }

        let _guard = builder.msg_unstaged(Kind::Build, "test helpers", target);
        t!(fs::create_dir_all(&dst));
        let mut cfg = cc::Build::new();

        // We may have found various cross-compilers a little differently due to our
        // extra configuration, so inform cc of these compilers. Note, though, that
        // on MSVC we still need cc's detection of env vars (ugh).
