use crate::prelude::*


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
                    // `std_cargo` actually does the wrong thing: it passes `--sysroot build/host/stage2`,
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

/// Rustdoc is special in various ways, which is why this step is different from `Crate`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateRustdoc {
    host: TargetSelection,
}

impl Step for CrateRustdoc {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.paths(&["src/librustdoc", "src/tools/rustdoc"])
    }

    fn make_run(run: RunConfig<'_>) {
        let builder = run.builder;

        builder.ensure(CrateRustdoc { host: run.target });
    }

    fn run(self, builder: &Builder<'_>) {
        let target = self.host;

        let compiler = if builder.download_rustc() {
            builder.compiler(builder.top_stage, target)
        } else {
            // Use the previous stage compiler to reuse the artifacts that are
            // created when running compiletest for tests/rustdoc. If this used
            // `compiler`, then it would cause rustdoc to be built *again*, which
            // isn't really necessary.
            builder.compiler_for(builder.top_stage, target, target)
        };
        // NOTE: normally `ensure(Rustc)` automatically runs `ensure(Std)` for us. However, when
        // using `download-rustc`, the rustc_private artifacts may be in a *different sysroot* from
        // the target rustdoc (`ci-rustc-sysroot` vs `stage2`). In that case, we need to ensure this
        // explicitly to make sure it ends up in the stage2 sysroot.
        builder.ensure(compile::Std::new(compiler, target));
        builder.ensure(compile::Rustc::new(compiler, target));

        let mut cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolRustc,
            target,
            builder.kind,
            "src/tools/rustdoc",
            SourceType::InTree,
            &[],
        );
        if self.host.contains("musl") {
            cargo.arg("'-Ctarget-feature=-crt-static'");
