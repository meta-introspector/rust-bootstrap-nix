use crate::prelude::*;


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
            eprintln!(
                "ERROR: `--stage 0` runs compiletest on the beta compiler, not your local changes, and will almost always cause tests to fail\nHELP: to test the compiler, use `--stage 1` instead\nHELP: to test the standard library, use `--stage 0 library/std` instead\nNOTE: if you're sure you want to do this, please open an issue as to why. In the meantime, you can override this with `COMPILETEST_FORCE_STAGE0=1`."
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
        let stage_id = if suite == "ui-fulldeps" && compiler.stage == 1 {
            // At stage 0 (stage - 1) we are using the beta compiler. Using `self.target` can lead finding
            // an incorrect compiler path on cross-targets, as the stage 0 beta compiler is always equal
            // to `build.build` in the configuration.
            let build = builder.build.build;

            compiler = builder.compiler(compiler.stage - 1, build);
            format!("stage{}-{}", compiler.stage + 1, build)
        } else {
            format!("stage{}-{}", compiler.stage, target)
        };

        if suite.ends_with("fulldeps") {
            builder.ensure(compile::Rustc::new(compiler, target));
        }

        if suite == "debuginfo" {
            builder.ensure(dist::DebuggerScripts {
                sysroot: builder.sysroot(compiler).to_path_buf(),
                host: target,
            });
        }

        // Also provide `rust_test_helpers` for the host.
        builder.ensure(TestHelpers { target: compiler.host });

        // ensure that `libproc_macro` is available on the host.
        if suite == "mir-opt" {
            builder.ensure(compile::Std::new_for_mir_opt_tests(compiler, compiler.host));
        } else {
            builder.ensure(compile::Std::new(compiler, compiler.host));
        }

        // As well as the target
        if suite != "mir-opt" {
            builder.ensure(TestHelpers { target });
        }

        let mut cmd = builder.tool_cmd(Tool::Compiletest);

        if suite == "mir-opt" {
            builder.ensure(compile::Std::new_for_mir_opt_tests(compiler, target));
        } else {
            builder.ensure(compile::Std::new(compiler, target));
        }

        builder.ensure(RemoteCopyLibs { compiler, target });

        // compiletest currently has... a lot of arguments, so let's just pass all
        // of them!

        cmd.arg("--compile-lib-path").arg(builder.rustc_libdir(compiler));
        cmd.arg("--run-lib-path").arg(builder.sysroot_target_libdir(compiler, target));
        cmd.arg("--rustc-path").arg(builder.rustc(compiler));

        // Minicore auxiliary lib for `no_core` tests that need `core` stubs in cross-compilation
        // scenarios.
        cmd.arg("--minicore-path")
            .arg(builder.src.join("tests").join("auxiliary").join("minicore.rs"));

        let is_rustdoc = suite.ends_with("rustdoc-ui") || suite.ends_with("rustdoc-js");

        if mode == "run-make" {
            let cargo_path = if builder.top_stage == 0 {
                // If we're using `--stage 0`, we should provide the bootstrap cargo.
                builder.initial_cargo.clone()
            } else {
                // We need to properly build cargo using the suitable stage compiler.

                let compiler = builder.download_rustc().then_some(compiler).unwrap_or_else(||
                    // HACK: currently tool stages are off-by-one compared to compiler stages, i.e. if
                    // you give `tool::Cargo` a stage 1 rustc, it will cause stage 2 rustc to be built
                    // and produce a cargo built with stage 2 rustc. To fix this, we need to chop off
                    // the compiler stage by 1 to align with expected `./x test run-make --stage N`
                    // behavior, i.e. we need to pass `N - 1` compiler stage to cargo. See also Miri
                    // which does a similar hack.
                    builder.compiler(builder.top_stage - 1, compiler.host));

                builder.ensure(tool::Cargo { compiler, target: compiler.host })
            };

            cmd.arg("--cargo-path").arg(cargo_path);
        }

        // Avoid depending on rustdoc when we don't need it.
        if mode == "rustdoc"
            || mode == "run-make"
            || (mode == "ui" && is_rustdoc)
            || mode == "js-doc-test"
            || mode == "rustdoc-json"
            || suite == "coverage-run-rustdoc"
        {
            cmd.arg("--rustdoc-path").arg(builder.rustdoc(compiler));
        }

        if mode == "rustdoc-json" {
            // Use the beta compiler for jsondocck
            let json_compiler = compiler.with_stage(0);
            cmd.arg("--jsondocck-path")
                .arg(builder.ensure(tool::JsonDocCk { compiler: json_compiler, target }));
            cmd.arg("--jsondoclint-path")
                .arg(builder.ensure(tool::JsonDocLint { compiler: json_compiler, target }));
        }

        if matches!(mode, "coverage-map" | "coverage-run") {
            let coverage_dump = builder.tool_exe(Tool::CoverageDump);
            cmd.arg("--coverage-dump-path").arg(coverage_dump);
        }

        cmd.arg("--src-base").arg(builder.src.join("tests").join(suite));
        cmd.arg("--build-base").arg(testdir(builder, compiler.host).join(suite));

        // When top stage is 0, that means that we're testing an externally provided compiler.
        // In that case we need to use its specific sysroot for tests to pass.
        let sysroot = if builder.top_stage == 0 {
            builder.initial_sysroot.clone()
        } else {
            builder.sysroot(compiler).to_path_buf()
        };
        cmd.arg("--sysroot-base").arg(sysroot);
        cmd.arg("--stage-id").arg(stage_id);
        cmd.arg("--suite").arg(suite);
        cmd.arg("--mode").arg(mode);
        cmd.arg("--target").arg(target.rustc_target_arg());
        cmd.arg("--host").arg(&*compiler.host.triple);
        cmd.arg("--llvm-filecheck").arg(builder.llvm_filecheck(builder.config.build));

        if builder.build.config.llvm_enzyme {
            cmd.arg("--has-enzyme");
        }

        if builder.config.cmd.bless() {
            cmd.arg("--bless");
        }

        if builder.config.cmd.force_rerun() {
            cmd.arg("--force-rerun");
        }

        let compare_mode = 
            builder.config.cmd.compare_mode().or_else(|| {
                if builder.config.test_compare_mode { self.compare_mode } else { None }
            });

        if let Some(ref pass) = builder.config.cmd.pass() {
            cmd.arg("--pass");
            cmd.arg(pass);
        }

        if let Some(ref run) = builder.config.cmd.run() {
            cmd.arg("--run");
            cmd.arg(run);
        }

        if let Some(ref nodejs) = builder.config.nodejs {
            cmd.arg("--nodejs").arg(nodejs);
        } else if mode == "js-doc-test" {
            panic!("need nodejs to run js-doc-test suite");
        }
        if let Some(ref npm) = builder.config.npm {
            cmd.arg("--npm").arg(npm);
        }
        if builder.config.rust_optimize_tests {
            cmd.arg("--optimize-tests");
        }
        if builder.config.rust_randomize_layout {
            cmd.arg("--rust-randomized-layout");
        }
        if builder.config.cmd.only_modified() {
            cmd.arg("--only-modified");
        }
        if let Some(compiletest_diff_tool) = &builder.config.compiletest_diff_tool {
            cmd.arg("--compiletest-diff-tool").arg(compiletest_diff_tool);
        }
    }
}
