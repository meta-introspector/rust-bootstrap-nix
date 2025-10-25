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

        let mut flags = if is_rustdoc { Vec::new() } else { vec!["-Crpath".to_string()] };
        flags.push(format!("-Cdebuginfo={}", builder.config.rust_debuginfo_level_tests));
        flags.extend(builder.config.cmd.compiletest_rustc_args().iter().map(|s| s.to_string()));

        if suite != "mir-opt" {
            if let Some(linker) = builder.linker(target) {
                cmd.arg("--target-linker").arg(linker);
            }
            if let Some(linker) = builder.linker(compiler.host) {
                cmd.arg("--host-linker").arg(linker);
            }
        }

        let mut hostflags = flags.clone();
        hostflags.push(format!("-Lnative={}", builder.test_helpers_out(compiler.host).display()));
        hostflags.extend(linker_flags(builder, compiler.host, LldThreads::No));
        for flag in hostflags {
            cmd.arg("--host-rustcflags").arg(flag);
        }

        let mut targetflags = flags;
        targetflags.push(format!("-Lnative={}", builder.test_helpers_out(target).display()));
        targetflags.extend(linker_flags(builder, compiler.host, LldThreads::No));
        for flag in targetflags {
            cmd.arg("--target-rustcflags").arg(flag);
        }

        cmd.arg("--python").arg(builder.python());

        if let Some(ref gdb) = builder.config.gdb {
            cmd.arg("--gdb").arg(gdb);
        }

        let lldb_exe = builder.config.lldb.clone().unwrap_or_else(|| PathBuf::from("lldb"));
        let lldb_version = command(&lldb_exe)
            .allow_failure()
            .arg("--version")
            .run_capture(builder)
            .stdout_if_ok()
            .and_then(|v| if v.trim().is_empty() { None } else { Some(v) });
        if let Some(ref vers) = lldb_version {
            cmd.arg("--lldb-version").arg(vers);
            let lldb_python_dir = command(&lldb_exe)
                .allow_failure()
                .arg("-P")
                .run_capture_stdout(builder)
                .stdout_if_ok()
                .map(|p| p.lines().next().expect("lldb Python dir not found").to_string());
            if let Some(ref dir) = lldb_python_dir {
                cmd.arg("--lldb-python-dir").arg(dir);
            }
        }

        if helpers::forcing_clang_based_tests() {
            let clang_exe = builder.llvm_out(target).join("bin").join("clang");
            cmd.arg("--run-clang-based-tests-with").arg(clang_exe);
        }

        for exclude in &builder.config.skip {
            cmd.arg("--skip");
            cmd.arg(exclude);
        }

        // Get paths from cmd args
        let paths = match &builder.config.cmd {
            Subcommand::Test { .. } => &builder.config.paths[..],
            _ => &[],
        };

        // Get test-args by striping suite path
        let mut test_args: Vec<&str> = paths
            .iter()
            .filter_map(|p| helpers::is_valid_test_suite_arg(p, suite_path, builder))
            .collect();

        test_args.append(&mut builder.config.test_args());

        // On Windows, replace forward slashes in test-args by backslashes
        // so the correct filters are passed to libtest
        if cfg!(windows) {
            let test_args_win: Vec<String> =
                test_args.iter().map(|s| s.replace('/', "\\")).collect();
            cmd.args(&test_args_win);
        } else {
            cmd.args(&test_args);
        }

        if builder.is_verbose() {
            cmd.arg("--verbose");
        }

        cmd.arg("--json");

        if builder.config.rustc_debug_assertions {
            cmd.arg("--with-rustc-debug-assertions");
        }

        if builder.config.std_debug_assertions {
            cmd.arg("--with-std-debug-assertions");
        }

        let mut llvm_components_passed = false;
        let mut copts_passed = false;
        if builder.config.llvm_enabled(compiler.host) {
            let llvm::LlvmResult { llvm_config, .. } =
                builder.ensure(llvm::Llvm { target: builder.config.build });
            if !builder.config.dry_run {
                let llvm_version =
                    command(&llvm_config).arg("--version").run_capture_stdout(builder).stdout();
                let llvm_components =
                    command(&llvm_config).arg("--components").run_capture_stdout(builder).stdout();
                // Remove trailing newline from llvm-config output.
                cmd.arg("--llvm-version")
                    .arg(llvm_version.trim())
                    .arg("--llvm-components")
                    .arg(llvm_components.trim());
                llvm_components_passed = true;
            }
            if !builder.is_rust_llvm(target) {
                // FIXME: missing Rust patches is not the same as being system llvm; we should rename the flag at some point.
                // Inspecting the tests with `// no-system-llvm` in src/test *looks* like this is doing the right thing, though.
                cmd.arg("--system-llvm");
            }

            // Tests that use compiler libraries may inherit the `-lLLVM` link
            // requirement, but the `-L` library path is not propagated across
            // separate compilations. We can add LLVM's library path to the
            // platform-specific environment variable as a workaround.
            if !builder.config.dry_run && suite.ends_with("fulldeps") {
                let llvm_libdir =
                    command(&llvm_config).arg("--libdir").run_capture_stdout(builder).stdout();
                add_link_lib_path(vec![llvm_libdir.trim().into()], &mut cmd);
            }

            if !builder.config.dry_run && matches!(mode, "run-make" | "coverage-run") {
                // The llvm/bin directory contains many useful cross-platform
                // tools. Pass the path to run-make tests so they can use them.
                // (The coverage-run tests also need these tools to process
                // coverage reports.)
                let llvm_bin_path = llvm_config
                    .parent()
                    .expect("Expected llvm-config to be contained in directory");
                assert!(llvm_bin_path.is_dir());
                cmd.arg("--llvm-bin-dir").arg(llvm_bin_path);
            }

            if !builder.config.dry_run && mode == "run-make" {
                // If LLD is available, add it to the PATH
                if builder.config.lld_enabled {
                    let lld_install_root =
                        builder.ensure(llvm::Lld { target: builder.config.build });

                    let lld_bin_path = lld_install_root.join("bin");

                    let old_path = env::var_os("PATH").unwrap_or_default();
                    let new_path = env::join_paths(
                        std::iter::once(lld_bin_path).chain(env::split_paths(&old_path)),
                    )
                    .expect("Could not add LLD bin path to PATH");
                    cmd.env("PATH", new_path);
                }
            }
        }

        // Only pass correct values for these flags for the `run-make` suite as it
        // requires that a C++ compiler was configured which isn't always the case.
        if !builder.config.dry_run && mode == "run-make" {
            cmd.arg("--cc")
                .arg(builder.cc(target))
                .arg("--cxx")
                .arg(builder.cxx(target).unwrap())
                .arg("--cflags")
                .arg(builder.cflags(target, GitRepo::Rustc, CLang::C).join(" "))
                .arg("--cxxflags")
                .arg(builder.cflags(target, GitRepo::Rustc, CLang::Cxx).join(" "));
            copts_passed = true;
            if let Some(ar) = builder.ar(target) {
                cmd.arg("--ar").arg(ar);
            }
        }

        if !llvm_components_passed {
            cmd.arg("--llvm-components").arg("");
        }
        if !copts_passed {
            cmd.arg("--cc")
                .arg("")
                .arg("--cxx")
                .arg("")
                .arg("--cflags")
                .arg("")
                .arg("--cxxflags")
                .arg("");
        }

        if builder.remote_tested(target) {
            cmd.arg("--remote-test-client").arg(builder.tool_exe(Tool::RemoteTestClient));
        } else if let Some(tool) = builder.runner(target) {
            cmd.arg("--runner").arg(tool);
        }

        if suite != "mir-opt" {
            // Running a C compiler on MSVC requires a few env vars to be set, to be
            // sure to set them here.
            //
            // Note that if we encounter `PATH` we make sure to append to our own `PATH`
            // rather than stomp over it.
            if !builder.config.dry_run && target.is_msvc() {
                for (k, v) in builder.cc.borrow()[&target].env() {
                    if k != "PATH" {
                        cmd.env(k, v);
                    }
                }
            }
        }

        // Special setup to enable running with sanitizers on MSVC.
        if !builder.config.dry_run
            && target.contains("msvc")
            && builder.config.sanitizers_enabled(target)
        {
            // Ignore interception failures: not all dlls in the process will have been built with
            // address sanitizer enabled (e.g., ntdll.dll).
            cmd.env("ASAN_WIN_CONTINUE_ON_INTERCEPTION_FAILURE", "1");
            // Add the address sanitizer runtime to the PATH - it is located next to cl.exe.
            let asan_runtime_path =
                builder.cc.borrow()[&target].path().parent().unwrap().to_path_buf();
            let old_path = cmd
                .get_envs()
                .find_map(|(k, v)| (k == "PATH").then_some(v))
                .flatten()
                .map_or_else(|| env::var_os("PATH").unwrap_or_default(), |v| v.to_owned());
            let new_path = env::join_paths(
                env::split_paths(&old_path).chain(std::iter::once(asan_runtime_path)),
            )
            .expect("Could not add ASAN runtime path to PATH");
            cmd.env("PATH", new_path);
        }

        // Some UI tests trigger behavior in rustc where it reads $CARGO and changes behavior if it exists.
        // To make the tests work that rely on it not being set, make sure it is not set.
        cmd.env_remove("CARGO");

        cmd.env("RUSTC_BOOTSTRAP", "1");
        // Override the rustc version used in symbol hashes to reduce the amount of normalization
        // needed when diffing test output.
        cmd.env("RUSTC_FORCE_RUSTC_VERSION", "compiletest");
        cmd.env("DOC_RUST_LANG_ORG_CHANNEL", builder.doc_rust_lang_org_channel());
        builder.add_rust_test_threads(&mut cmd);

        if builder.config.sanitizers_enabled(target) {
            cmd.env("RUSTC_SANITIZER_SUPPORT", "1");
        }

        if builder.config.profiler_enabled(target) {
            cmd.arg("--profiler-runtime");
        }

        cmd.env("RUST_TEST_TMPDIR", builder.tempdir());

        cmd.arg("--adb-path").arg("adb");
        cmd.arg("--adb-test-dir").arg(ADB_TEST_DIR);
        if target.contains("android") && !builder.config.dry_run {
            // Assume that cc for this target comes from the android sysroot
            cmd.arg("--android-cross-path")
                .arg(builder.cc(target).parent().unwrap().parent().unwrap());
        } else {
            cmd.arg("--android-cross-path").arg("");
        }

        if builder.config.cmd.rustfix_coverage() {
            cmd.arg("--rustfix-coverage");
        }

        cmd.arg("--channel").arg(&builder.config.channel);

        if !builder.config.omit_git_hash {
            cmd.arg("--git-hash");
        }

        let git_config = builder.config.git_config();
        cmd.arg("--git-repository").arg(git_config.git_repository);
        cmd.arg("--nightly-branch").arg(git_config.nightly_branch);
        cmd.arg("--git-merge-commit-email").arg(git_config.git_merge_commit_email);
        cmd.force_coloring_in_ci();

        #[cfg(feature = "build-metrics")]
        builder.metrics.begin_test_suite(
            build_helper::metrics::TestSuiteMetadata::Compiletest {
                suite: suite.into(),
                mode: mode.into(),
                compare_mode: None,
                target: self.target.triple.to_string(),
                host: self.compiler.host.triple.to_string(),
                stage: self.compiler.stage,
            },
            builder,
        );

        let _group = builder.msg(
            Kind::Test,
            compiler.stage,
            format!("compiletest suite={suite} mode={mode}"),
            compiler.host,
            target,
        );
        try_run_tests(builder, &mut cmd, false);

        if let Some(compare_mode) = compare_mode {
            cmd.arg("--compare-mode").arg(compare_mode);

            #[cfg(feature = "build-metrics")]
            builder.metrics.begin_test_suite(
                build_helper::metrics::TestSuiteMetadata::Compiletest {
                    suite: suite.into(),
                    mode: mode.into(),
                    compare_mode: Some(compare_mode.into()),
                    target: self.target.triple.to_string(),
                    host: self.compiler.host.triple.to_string(),
                    stage: self.compiler.stage,
                },
                builder,
            );

            builder.info(&format!(
                "Check compiletest suite={} mode={} compare_mode={} ({} -> {})",
                suite, mode, compare_mode, &compiler.host, target
            ));
            let _time = helpers::timeit(builder);
            try_run_tests(builder, &mut cmd, false);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BookTest {
    compiler: Compiler,
    path: PathBuf,
    name: &'static str,
    is_ext_doc: bool,
}

impl Step for BookTest {
    type Output = ();
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    /// Runs the documentation tests for a book in `src/doc`.
    ///
    /// This uses the `rustdoc` that sits next to `compiler`.
    fn run(self, builder: &Builder<'_>) {
        // External docs are different from local because:
        // - Some books need pre-processing by mdbook before being tested.
        // - They need to save their state to toolstate.
        // - They are only tested on the "checktools" builders.
        //
        // The local docs are tested by default, and we don't want to pay the
        // cost of building mdbook, so they use `rustdoc --test` directly.
        // Also, the unstable book is special because SUMMARY.md is generated,
        // so it is easier to just run `rustdoc` on its files.
        if self.is_ext_doc {
            self.run_ext_doc(builder);
        } else {
            self.run_local_doc(builder);
        }
    }
}

impl BookTest {
    /// This runs the equivalent of `mdbook test` (via the rustbook wrapper)
    /// which in turn runs `rustdoc --test` on each file in the book.
    fn run_ext_doc(self, builder: &Builder<'_>) {
        let compiler = self.compiler;

        builder.ensure(compile::Std::new(compiler, compiler.host));

        // mdbook just executes a binary named "rustdoc", so we need to update
        // PATH so that it points to our rustdoc.
        let mut rustdoc_path = builder.rustdoc(compiler);
        rustdoc_path.pop();
        let old_path = env::var_os("PATH").unwrap_or_default();
        let new_path = env::join_paths(iter::once(rustdoc_path).chain(env::split_paths(&old_path)))
            .expect("could not add rustdoc to PATH");

        let mut rustbook_cmd = builder.tool_cmd(Tool::Rustbook);
