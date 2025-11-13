use crate::prelude::*;



            for target in ["x86_64-apple-darwin", "i686-unknown-linux-musl"] {
                let target = TargetSelection::from_user(target);
                let panic_abort_target = builder.ensure(MirOptPanicAbortSyntheticTarget {
                    compiler: self.common.compiler,
                    base: target,
                });
                run(panic_abort_target);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Compiletest {
    compiler: Compiler,
    target: TargetSelection,
    mode: &'static str,
    suite: &'static str,
    path: &'static str,
    compare_mode: Option<&'static str>,
}

impl Step for Compiletest {
    type Output = ();

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    /// Executes the `compiletest` tool to run a suite of tests.
    ///
    /// Compiles all tests with `compiler` for `target` with the specified
    /// compiletest `mode` and `suite` arguments. For example `mode` can be
    /// "run-pass" or `suite` can be something like `debuginfo`.
    fn run(self, builder: &Builder<'_>) {
        if builder.doc_tests == DocTests::Only {
            return;
        }

        if builder.top_stage == 0 && env::var("COMPILETEST_FORCE_STAGE0").is_err() {
            eprintln!("\
ERROR: `--stage 0` runs compiletest on the beta compiler, not your local changes, and will almost always cause tests to fail
HELP: to test the compiler, use `--stage 1` instead
HELP: to test the standard library, use `--stage 0 library/std` instead
NOTE: if you're sure you want to do this, please open an issue as to why. In the meantime, you can override this with `COMPILETEST_FORCE_STAGE0=1`."
            );
            crate::exit!(1);
        }

        let mut compiler = self.compiler;
        let target = self.target;
        let mode = self.mode;
        let suite = self.suite;

        // Path for test suite
        let suite_path = self.path;

        // Skip codegen tests if they aren't enabled in configuration.
        if !builder.config.codegen_tests && suite == "codegen" {
            return;
        }

        // Support stage 1 ui-fulldeps. This is somewhat complicated: ui-fulldeps tests for the most
        // part test the *API* of the compiler, not how it compiles a given file. As a result, we
        // can run them against the stage 1 sources as long as we build them with the stage 0
        // bootstrap compiler.
        // NOTE: Only stage 1 is special cased because we need the rustc_private artifacts to match the
        // running compiler in stage 2 when plugins run.
