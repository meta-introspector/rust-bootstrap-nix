HELP: if you are testing a beta branch, set `rust.channel` to \"beta\" in the `config.toml` file
HELP: to skip test's attempt to check tidiness, pass `--skip src/tools/tidy` to `x.py test`",
                    PATH = inferred_rustfmt_dir.display(),
                    CHAN = builder.config.channel,
                );
                crate::exit!(1);
            }
            let all = false;
            crate::core::build_steps::format::format(builder, !builder.config.cmd.bless(), all, &[
            ]);
        }

        builder.info("tidy check");
        cmd.delay_failure().run(builder);

        builder.info("x.py completions check");
        let [bash, zsh, fish, powershell] = ["x.py.sh", "x.py.zsh", "x.py.fish", "x.py.ps1"]
            .map(|filename| builder.src.join("src/etc/completions").join(filename));
        if builder.config.cmd.bless() {
            builder.ensure(crate::core::build_steps::run::GenerateCompletions);
        } else if get_completion(shells::Bash, &bash).is_some()
            || get_completion(shells::Fish, &fish).is_some()
            || get_completion(shells::PowerShell, &powershell).is_some()
            || crate::flags::get_completion(shells::Zsh, &zsh).is_some()
        {
            eprintln!(
                "x.py completions were changed; run `x.py run generate-completions` to update them"
            );
            crate::exit!(1);
        }
    }

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let default = run.builder.doc_tests != DocTests::Only;
        run.path("src/tools/tidy").default_condition(default)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Tidy);
    }
}

fn testdir(builder: &Builder<'_>, host: TargetSelection) -> PathBuf {
    builder.out.join(host).join("test")
}

macro_rules! default_test {
    ($name:ident { path: $path:expr, mode: $mode:expr, suite: $suite:expr }) => {
        test!($name { path: $path, mode: $mode, suite: $suite, default: true, host: false });
    };
}

macro_rules! default_test_with_compare_mode {
    ($name:ident { path: $path:expr, mode: $mode:expr, suite: $suite:expr,
                   compare_mode: $compare_mode:expr }) => {
        test_with_compare_mode!($name {
            path: $path,
            mode: $mode,
            suite: $suite,
            default: true,
            host: false,
            compare_mode: $compare_mode
        });
    };
}

macro_rules! host_test {
    ($name:ident { path: $path:expr, mode: $mode:expr, suite: $suite:expr }) => {
        test!($name { path: $path, mode: $mode, suite: $suite, default: true, host: true });
    };
}

macro_rules! test {
    ($name:ident { path: $path:expr, mode: $mode:expr, suite: $suite:expr, default: $default:expr,
                   host: $host:expr }) => {
        test_definitions!($name {
            path: $path,
            mode: $mode,
            suite: $suite,
            default: $default,
            host: $host,
            compare_mode: None
        });
    };
}

macro_rules! test_with_compare_mode {
    ($name:ident { path: $path:expr, mode: $mode:expr, suite: $suite:expr, default: $default:expr,
                   host: $host:expr, compare_mode: $compare_mode:expr }) => {
        test_definitions!($name {
            path: $path,
            mode: $mode,
            suite: $suite,
            default: $default,
            host: $host,
            compare_mode: Some($compare_mode)
        });
    };
}

macro_rules! test_definitions {
    ($name:ident {
        path: $path:expr,
        mode: $mode:expr,
        suite: $suite:expr,
        default: $default:expr,
        host: $host:expr,
        compare_mode: $compare_mode:expr
    }) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name {
            pub compiler: Compiler,
            pub target: TargetSelection,
        }

        impl Step for $name {
            type Output = ();
            const DEFAULT: bool = $default;
            const ONLY_HOSTS: bool = $host;

            fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
                run.suite_path($path)
            }

            fn make_run(run: RunConfig<'_>) {
                let compiler = run.builder.compiler(run.builder.top_stage, run.build_triple());

                run.builder.ensure($name { compiler, target: run.target });
            }

            fn run(self, builder: &Builder<'_>) {
                builder.ensure(Compiletest {
                    compiler: self.compiler,
                    target: self.target,
                    mode: $mode,
                    suite: $suite,
                    path: $path,
                    compare_mode: $compare_mode,
                })
            }
        }
    };
}

/// Declares an alias for running the [`Coverage`] tests in only one mode.
/// Adapted from [`test_definitions`].
macro_rules! coverage_test_alias {
    ($name:ident {
        alias_and_mode: $alias_and_mode:expr, // &'static str
        default: $default:expr, // bool
        only_hosts: $only_hosts:expr $(,)? // bool
    }) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name {
            pub compiler: Compiler,
            pub target: TargetSelection,
        }

        impl $name {
            const MODE: &'static str = $alias_and_mode;
        }
