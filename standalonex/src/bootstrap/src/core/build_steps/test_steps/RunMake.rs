use crate::prelude::*


///
/// This combined step runs the specified tests (or all of `tests/coverage`)
/// in both "coverage-map" and "coverage-run" modes.
///
/// Used by:
/// - `x test coverage`
/// - `x test tests/coverage`
/// - `x test tests/coverage/trivial.rs` (etc)
///
/// (Each individual mode also has its own step that will run the tests in
/// just that mode.)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coverage {
    pub common: common_test_fields::CommonTestFields,
}

impl Coverage {
    const PATH: &'static str = "tests/coverage";
    const SUITE: &'static str = "coverage";

    /// Runs the coverage test suite (or a user-specified subset) in one mode.
    ///
    /// This same function is used by the multi-mode step ([`Coverage`]) and by
    /// the single-mode steps ([`CoverageMap`] and [`CoverageRun`]), to help
    /// ensure that they all behave consistently with each other, regardless of
    /// how the coverage tests have been invoked.
    fn run_coverage_tests(
        builder: &Builder<'_>,
        compiler: Compiler,
        target: TargetSelection,
        mode: &'static str,
    ) {
        // Like many other test steps, we delegate to a `Compiletest` step to
        // actually run the tests. (See `test_definitions!`.)
        builder.ensure(Compiletest {
            compiler,
            target,
            mode,
            suite: Self::SUITE,
            path: Self::PATH,
            compare_mode: None,
        });
    }
}

impl Step for Coverage {
    type Output = ();
    /// We rely on the individual CoverageMap/CoverageRun steps to run themselves.
    const DEFAULT: bool = false;
    /// When manually invoked, try to run as much as possible.
    /// Compiletest will automatically skip the "coverage-run" tests if necessary.
    const ONLY_HOSTS: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        // Take responsibility for command-line paths within `tests/coverage`.
        run.suite_path(Self::PATH)
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.build_triple());

        run.builder.ensure(Coverage {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.build_triple(),
                compiler,
                target: run.target,
            },
        });
    }

    fn run(self, builder: &Builder<'_>) {
        // Run the specified coverage tests (possibly all of them) in both modes.
        Self::run_coverage_tests(builder, self.common.compiler, self.common.target, CoverageMap::MODE);
        Self::run_coverage_tests(builder, self.common.compiler, self.common.target, CoverageRun::MODE);
    }
}

// Runs `tests/coverage` in "coverage-map" mode only.
// Used by `x test` and `x test coverage-map`.
coverage_test_alias!(CoverageMap {
    alias_and_mode: "coverage-map",
    default: true,
    only_hosts: false,
});
// Runs `tests/coverage` in "coverage-run" mode only.
// Used by `x test` and `x test coverage-run`.
coverage_test_alias!(CoverageRun {
    alias_and_mode: "coverage-run",
    default: true,
    // Compiletest knows how to automatically skip these tests when cross-compiling,
