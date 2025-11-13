use crate::prelude::*;


#[macro_export]
#[macro_export]
macro_rules! default_test {
    ($name:ident { path: $path:expr, mode: $mode:expr, suite: $suite:expr }) => {
        test!($name { path: $path, mode: $mode, suite: $suite, default: true, host: false });
    };
}

#[macro_export]
#[macro_export]
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

#[macro_export]
#[macro_export]
macro_rules! host_test {
    ($name:ident { path: $path:expr, mode: $mode:expr, suite: $suite:expr }) => {
        test!($name { path: $path, mode: $mode, suite: $suite, default: true, host: true });
    };
}

#[macro_export]
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

#[macro_export]
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

#[macro_export]
/// Declares an alias for running the [`Coverage`] tests in only one mode.
/// Adapted from [`test_definitions`].
#[macro_export]
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

        impl Step for $name {
            type Output = ();
            const DEFAULT: bool = $default;
            const ONLY_HOSTS: bool = $only_hosts;

            fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
                // Register the mode name as a command-line alias.
                // This enables `x test coverage-map` and `x test coverage-run`.
                run.alias($alias_and_mode)
            }

            fn make_run(run: RunConfig<'_>) {
                let compiler = run.builder.compiler(run.builder.top_stage, run.build_triple());

                run.builder.ensure($name { compiler, target: run.target });
            }

            fn run(self, builder: &Builder<'_>) {
                crate::coverage::Coverage::run_coverage_tests(builder, self.compiler, self.target, Self::MODE);
            }
        }
    };
}

#[macro_export]
macro_rules! test_book {
    ($(
        $name:ident, $path:expr, $book_name:expr,
        default=$default:expr
        $(,submodules = $submodules:expr)?
        ;
    )+) => {
        $(
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct $name {
                compiler: Compiler,
            }

            impl Step for $name {
                type Output = ();
                const DEFAULT: bool = $default;
                const ONLY_HOSTS: bool = true;

                fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
                    run.path($path)
                }

                fn make_run(run: RunConfig<'_>) {
                    run.builder.ensure($name {
                        compiler: run.builder.compiler(run.builder.top_stage, run.target),
                    });
                }

                fn run(self, builder: &Builder<'_>) {
                    $(
                        for submodule in $submodules {
                            builder.require_submodule(submodule, None);
                        }
                    )*
                    builder.ensure(crate::book_test::BookTest {
                        compiler: self.compiler,
                        path: PathBuf::from($path),
                        name: $book_name,
                        is_ext_doc: !$default,
                    });
                }
            }
        )+
    }
}
