use crate::prelude::*;


            compiler: host_compiler,
            target: host,
            extra_features: Vec::new(),
        });

        // We also need sysroots, for Miri and for the host (the latter for build scripts).
        // This is for the tests so everything is done with the target compiler.
        let miri_sysroot = Miri::build_miri_sysroot(builder, target_compiler, target);
        builder.ensure(compile::Std::new(target_compiler, host));
        let host_sysroot = builder.sysroot(target_compiler);

        // Miri has its own "target dir" for ui test dependencies. Make sure it gets cleared when
        // the sysroot gets rebuilt, to avoid "found possibly newer version of crate `std`" errors.
        if !builder.config.dry_run {
            let ui_test_dep_dir = builder.stage_out(host_compiler, Mode::ToolStd).join("miri_ui");
            // The mtime of `miri_sysroot` changes when the sysroot gets rebuilt (also see
            // <https://github.com/RalfJung/rustc-build-sysroot/commit/10ebcf60b80fe2c3dc765af0ff19fdc0da4b7466>).
            // We can hence use that directly as a signal to clear the ui test dir.
            builder.clear_if_dirty(&ui_test_dep_dir, &miri_sysroot);
        }

        // Run `cargo test`.
        // This is with the Miri crate, so it uses the host compiler.
        let mut cargo = tool::prepare_tool_cargo(
            builder,
            host_compiler,
            Mode::ToolRustc,
            host,
            Kind::Test,
            "src/tools/miri",
            SourceType::InTree,
            &[],
        );

        cargo.add_rustc_lib_path(builder);

        // We can NOT use `run_cargo_test` since Miri's integration tests do not use the usual test
        // harness and therefore do not understand the flags added by `add_flags_and_try_run_test`.
        let mut cargo = prepare_cargo_test(cargo, &[], &[], "miri", host_compiler, host, builder);

        // miri tests need to know about the stage sysroot
        cargo.env("MIRI_SYSROOT", &miri_sysroot);
        cargo.env("MIRI_HOST_SYSROOT", &host_sysroot);
        cargo.env("MIRI", &miri);

        // Set the target.
        cargo.env("MIRI_TEST_TARGET", target.rustc_target_arg());

        {
            let _guard = builder.msg_sysroot_tool(Kind::Test, stage, "miri", host, target);
            let _time = helpers::timeit(builder);
            cargo.run(builder);
        }

        // Run it again for mir-opt-level 4 to catch some miscompilations.
        if builder.config.test_args().is_empty() {
            cargo.env("MIRIFLAGS", "-O -Zmir-opt-level=4 -Cdebug-assertions=yes");
            // Optimizations can change backtraces
            cargo.env("MIRI_SKIP_UI_CHECKS", "1");
            // `MIRI_SKIP_UI_CHECKS` and `RUSTC_BLESS` are incompatible
            cargo.env_remove("RUSTC_BLESS");
            // Optimizations can change error locations and remove UB so don't run `fail` tests.
            cargo.args(["tests/pass", "tests/panic"]);

            {
                let _guard = builder.msg_sysroot_tool(
                    Kind::Test,
                    stage,
                    "miri (mir-opt-level 4)",
                    host,
                    target,
                );
                let _time = helpers::timeit(builder);
                cargo.run(builder);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CargoMiri {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for CargoMiri {
    type Output = ();
    const ONLY_HOSTS: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/tools/miri/cargo-miri")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.target);
        run.builder.ensure(CargoMiri {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.builder.build.build,
                compiler,
                target: run.target,
            },
        });
    }

    /// Tests `cargo miri test`.
    fn run(self, builder: &Builder<'_>) {
        let host = self.common.host;
        let target = self.common.target;
        let stage = self.common.stage;
        if stage == 0 {
            eprintln!("cargo-miri cannot be tested at stage 0");
            std::process::exit(1);
        }

        // This compiler runs on the host, we'll just use it for the target.
        let compiler = builder.compiler(stage, host);

        // Run `cargo miri test`.
        // This is just a smoke test (Miri's own CI invokes this in a bunch of different ways and ensures
        // that we get the desired output), but that is sufficient to make sure that the libtest harness
        // itself executes properly under Miri, and that all the logic in `cargo-miri` does not explode.
        let mut cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolStd, // it's unclear what to use here, we're not building anything just doing a smoke test!
            target,
            Kind::MiriTest,
            "src/tools/miri/test-cargo-miri",
            SourceType::Submodule,
            &[],
