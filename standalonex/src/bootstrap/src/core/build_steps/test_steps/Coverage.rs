use crate::prelude::*


    // but skipping the whole step here makes it clearer that they haven't run at all.
    only_hosts: true,
});

host_test!(CoverageRunRustdoc {
    path: "tests/coverage-run-rustdoc",
    mode: "coverage-run",
    suite: "coverage-run-rustdoc"
});

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MirOpt {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for MirOpt {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.suite_path("tests/mir-opt")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.build_triple());
        run.builder.ensure(MirOpt {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.build_triple(),
                compiler,
                target: run.target,
            },
        });
    }

    fn run(self, builder: &Builder<'_>) {
        let run = |target| {
            builder.ensure(Compiletest {
                compiler: self.common.compiler,
                target,
                mode: "mir-opt",
                suite: "mir-opt",
                path: "tests/mir-opt",
                compare_mode: None,
            })
        };

        run(self.common.target);

        // Run more targets with `--bless`. But we always run the host target first, since some
        // tests use very specific `only` clauses that are not covered by the target set below.
        if builder.config.cmd.bless() {
            // All that we really need to do is cover all combinations of 32/64-bit and unwind/abort,
            // but while we're at it we might as well flex our cross-compilation support. This
            // selection covers all our tier 1 operating systems and architectures using only tier
            // 1 targets.

            for target in ["aarch64-unknown-linux-gnu", "i686-pc-windows-msvc"] {
                run(TargetSelection::from_user(target));
            }
