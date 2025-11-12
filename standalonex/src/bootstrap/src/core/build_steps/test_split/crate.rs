#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Crate {
    pub compiler: Compiler,
    pub target: TargetSelection,
    pub mode: Mode,
    pub crates: Vec<String>,
}

impl Step for Crate {
    type Output = ();
    const DEFAULT: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.crate_or_deps("sysroot")
    }

    fn make_run(run: RunConfig<'_>) {
        let builder = run.builder;
        let host = run.build_triple();
        let compiler = builder.compiler_for(builder.top_stage, host, host);
        let crates = run
            .paths
            .iter()
            .map(|p| builder.crate_paths[&p.assert_single_path().path].clone())
            .collect();

        builder.ensure(Crate { compiler, target: run.target, mode: Mode::Std, crates });
    }

    /// Runs all unit tests plus documentation tests for a given crate defined
    /// by a `Cargo.toml` (single manifest)
    ///
    /// This is what runs tests for crates like the standard library, compiler, etc.
    /// It essentially is the driver for running `cargo test`.
    ///
    /// Currently this runs all tests for a DAG by passing a bunch of `-p foo`
    /// arguments, and those arguments are discovered from `cargo metadata`.
    fn run(self, builder: &Builder<'_>) {
        let compiler = self.compiler;
        let target = self.target;
        let mode = self.mode;

        // Prepare sysroot
        // See [field@compile::Std::force_recompile].
        builder.ensure(compile::Std::force_recompile(compiler, compiler.host));

        // If we're not doing a full bootstrap but we're testing a stage2
        // version of libstd, then what we're actually testing is the libstd
        // produced in stage1. Reflect that here by updating the compiler that
        // we're working with automatically.
        let compiler = builder.compiler_for(compiler.stage, compiler.host, target);

        let mut cargo = if builder.kind == Kind::Miri {
            if builder.top_stage == 0 {
                eprintln!("ERROR: `x.py miri` requires stage 1 or higher");
                std::process::exit(1);
            }

            // Build `cargo miri test` command
            // (Implicitly prepares target sysroot)
            let mut cargo = builder::Cargo::new(
                builder,
                compiler,
                mode,
                SourceType::InTree,
                target,
                Kind::MiriTest,
            );
            // This hack helps bootstrap run standard library tests in Miri. The issue is as
            // follows: when running `cargo miri test` on libcore, cargo builds a local copy of core
            // and makes it a dependency of the integration test crate. This copy duplicates all the
            // lang items, so the build fails. (Regular testing avoids this because the sysroot is a
            // literal copy of what `cargo build` produces, but since Miri builds its own sysroot
            // this does not work for us.) So we need to make it so that the locally built libcore
            // contains all the items from `core`, but does not re-define them -- we want to replace
            // the entire crate but a re-export of the sysroot crate. We do this by swapping out the
            // source file: if `MIRI_REPLACE_LIBRS_IF_NOT_TEST` is set and we are building a
            // `lib.rs` file, and a `lib.miri.rs` file exists in the same folder, we build that
            // instead. But crucially we only do that for the library, not the test builds.
            cargo.env("MIRI_REPLACE_LIBRS_IF_NOT_TEST", "1");
            cargo
        } else {
            // Also prepare a sysroot for the target.
            if builder.config.build != target {
                builder.ensure(compile::Std::force_recompile(compiler, target));
                builder.ensure(RemoteCopyLibs { compiler, target });
            }

            // Build `cargo test` command
            builder::Cargo::new(builder, compiler, mode, SourceType::InTree, target, builder.kind)
        };

        match mode {
            Mode::Std => {
                if builder.kind == Kind::Miri {
                    // We can't use `std_cargo` as that uses `optimized-compiler-builtins` which
                    // needs host tools for the given target. This is similar to what `compile::Std`
                    // does when `is_for_mir_opt_tests` is true. There's probably a chance for
                    // de-duplication here... `std_cargo` should support a mode that avoids needing
                    // host tools.
                    cargo
                        .arg("--manifest-path")
                        .arg(builder.src.join("library/sysroot/Cargo.toml"));
                } else {
                    compile::std_cargo(builder, target, compiler.stage, &mut cargo);
                    // `std-cargo` actually does the wrong thing: it passes `--sysroot build/host/stage2`,
                    // but we want to use the force-recompile std we just built in `build/host/stage2-test-sysroot`.
                    // Override it.
                    if builder.download_rustc() && compiler.stage > 0 {
                        let sysroot = builder
                            .out
                            .join(compiler.host)
                            .join(format!("stage{}-test-sysroot", compiler.stage));
                        cargo.env("RUSTC_SYSROOT", sysroot);
                    }
                }
            }
            Mode::Rustc => {
                compile::rustc_cargo(builder, &mut cargo, target, &compiler, &self.crates);
            }
            _ => panic!("can only test libraries"),
        };

        run_cargo_test(
            cargo,
            &[],
            &self.crates,
            &self.crates[0],
            &*crate_description(&self.crates),
            compiler,
            target,
            builder,
        );
    }
}
