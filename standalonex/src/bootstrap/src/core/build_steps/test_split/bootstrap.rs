use crate::prelude::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bootstrap;

impl Step for Bootstrap {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    /// Tests the build system itself.
    fn run(self, builder: &Builder<'_>) {
        let host = builder.config.build;
        let compiler = builder.compiler(0, host);
        let _guard = builder.msg(Kind::Test, 0, "bootstrap", host, host);

        // Some tests require cargo submodule to be present.
        builder.build.require_submodule("src/tools/cargo", None);

        let mut check_bootstrap = command(builder.python());
        check_bootstrap
            .args(["-m", "unittest", "bootstrap_test.py"])
            .env("BUILD_DIR", &builder.out)
            .env("BUILD_PLATFORM", builder.build.build.triple)
            .env("BOOTSTRAP_TEST_RUSTC_BIN", &builder.initial_rustc)
            .env("BOOTSTRAP_TEST_CARGO_BIN", &builder.initial_cargo)
            .current_dir(builder.src.join("src/bootstrap/"));
        // NOTE: we intentionally don't pass test_args here because the args for unittest and cargo test are mutually incompatible.
        // Use `python -m unittest` manually if you want to pass arguments.
        check_bootstrap.delay_failure().run(builder);

        let mut cmd = command(&builder.initial_cargo);
        cmd.arg("test")
            .args(["--features", "bootstrap-self-test"])
            .current_dir(builder.src.join("src/bootstrap"))
            .env("RUSTFLAGS", "-Cdebuginfo=2")
            .env("CARGO_TARGET_DIR", builder.out.join("bootstrap"))
            .env("RUSTC_BOOTSTRAP", "1")
            .env("RUSTDOC", builder.rustdoc(compiler))
            .env("RUSTC", &builder.initial_rustc);
        if let Some(flags) = option_env!("RUSTFLAGS") {
            // Use the same rustc flags for testing as for "normal" compilation,
            // so that Cargo doesn’t recompile the entire dependency graph every time:
            // https://github.com/rust-lang/rust/issues/49215
            cmd.env("RUSTFLAGS", flags);
        }
        // bootstrap tests are racy on directory creation so just run them one at a time.
        // Since there's not many this shouldn't be a problem.
        run_cargo_test(cmd, &["--test-threads=1"], &[], "bootstrap", None, compiler, host, builder);
    }

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/bootstrap")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Bootstrap);
    }
}
