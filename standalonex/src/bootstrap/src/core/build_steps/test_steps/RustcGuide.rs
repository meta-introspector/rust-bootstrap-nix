        let crates = run.make_run_crates(Alias::Compiler);

        builder.ensure(CrateLibrustc { compiler, target: run.target, crates });
    }

    fn run(self, builder: &Builder<'_>) {
        builder.ensure(compile::Std::new(self.compiler, self.target));

        builder.ensure(Crate {
            compiler: self.compiler,
            target: self.target,
            mode: Mode::Rustc,
            crates: self.crates,
        });
    }
}

/// Given a `cargo test` subcommand, add the appropriate flags and run it.
///
/// Returns whether the test succeeded.
#[allow(clippy::too_many_arguments)] // FIXME: reduce the number of args and remove this.
fn run_cargo_test<'a>(
    cargo: impl Into<BootstrapCommand>,
    libtest_args: &[&str],
    crates: &[String],
    primary_crate: &str,
    description: impl Into<Option<&'a str>>,
    compiler: Compiler,
    target: TargetSelection,
    builder: &Builder<'_>,
) -> bool {
    let mut cargo =
        prepare_cargo_test(cargo, libtest_args, crates, primary_crate, compiler, target, builder);
