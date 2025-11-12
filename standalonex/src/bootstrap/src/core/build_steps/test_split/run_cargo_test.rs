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
    let _time = helpers::timeit(builder);
    let _group = description.into().and_then(|what| {
        builder.msg_sysroot_tool(Kind::Test, compiler.stage, what, compiler.host, target)
    });

    #[cfg(feature = "build-metrics")]
    builder.metrics.begin_test_suite(
        build_helper::metrics::TestSuiteMetadata::CargoPackage {
            crates: crates.iter().map(|c| c.to_string()).collect(),
            target: target.triple.to_string(),
            host: compiler.host.triple.to_string(),
            stage: compiler.stage,
        },
        builder,
    );
    add_flags_and_try_run_tests(builder, &mut cargo)
}
