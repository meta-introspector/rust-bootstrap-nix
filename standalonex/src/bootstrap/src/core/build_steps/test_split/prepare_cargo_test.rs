use crate::prelude::*


/// Given a `cargo test` subcommand, pass it the appropriate test flags given a `builder`.
fn prepare_cargo_test(
    cargo: impl Into<BootstrapCommand>,
    libtest_args: &[&str],
    crates: &[String],
    primary_crate: &str,
    compiler: Compiler,
    target: TargetSelection,
    builder: &Builder<'_>,
) -> BootstrapCommand {
    let mut cargo = cargo.into();

    // Propagate `--bless` if it has not already been set/unset
    // Any tools that want to use this should bless if `RUSTC_BLESS` is set to
    // anything other than `0`.
    if builder.config.cmd.bless() && !cargo.get_envs().any(|v| v.0 == "RUSTC_BLESS") {
        cargo.env("RUSTC_BLESS", "Gesundheit");
    }

    // Pass in some standard flags then iterate over the graph we've discovered
    // in `cargo metadata` with the maps above and figure out what `-p`
    // arguments need to get passed.
    if builder.kind == Kind::Test && !builder.fail_fast {
        cargo.arg("--no-fail-fast");
    }
    match builder.doc_tests {
        DocTests::Only => {
            cargo.arg("--doc");
        }
        DocTests::No => {
            let krate = &builder
                .crates
                .get(primary_crate)
                .unwrap_or_else(|| panic!("missing crate {primary_crate}"));
            if krate.has_lib {
                cargo.arg("--lib");
            }
            cargo.args(["--bins", "--examples", "--tests", "--benches"]);
        }
        DocTests::Yes => {}
    }

    for krate in crates {
        cargo.arg("-p").arg(krate);
    }

    cargo.arg("--").args(builder.config.test_args()).args(libtest_args);
    if !builder.config.verbose_tests {
        cargo.arg("--quiet");
    }

    // The tests are going to run with the *target* libraries, so we need to
    // ensure that those libraries show up in the LD_LIBRARY_PATH equivalent.
    //
    // Note that to run the compiler we need to run with the *host* libraries,
    // but our wrapper scripts arrange for that to be the case anyway.
    //
    // We skip everything on Miri as then this overwrites the libdir set up
    // by `Cargo::new` and that actually makes things go wrong.
    if builder.kind != Kind::Miri {
        let mut dylib_path = dylib_path();
        dylib_path.insert(0, PathBuf::from(&*builder.sysroot_target_libdir(compiler, target)));
        cargo.env(dylib_path_var(), env::join_paths(&dylib_path).unwrap());
    }

    if builder.remote_tested(target) {
        cargo.env(
            format!("CARGO_TARGET_{}_RUNNER", envify(&target.triple)),
            format!("{} run 0", builder.tool_exe(Tool::RemoteTestClient).display()),
        );
    } else if let Some(tool) = builder.runner(target) {
        cargo.env(format!("CARGO_TARGET_{}_RUNNER", envify(&target.triple)), tool);
    }

    cargo
}
