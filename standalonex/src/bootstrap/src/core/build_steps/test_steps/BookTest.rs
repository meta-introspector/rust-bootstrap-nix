use crate::prelude::*;


        let path = builder.src.join(&self.path);
        // Books often have feature-gated example text.
        rustbook_cmd.env("RUSTC_BOOTSTRAP", "1");
        rustbook_cmd.env("PATH", new_path).arg("test").arg(path);
        builder.add_rust_test_threads(&mut rustbook_cmd);
        let _guard = builder.msg(
            Kind::Test,
            compiler.stage,
            format_args!("mdbook {}", self.path.display()),
            compiler.host,
            compiler.host,
        );
        let _time = helpers::timeit(builder);
        let toolstate = if rustbook_cmd.delay_failure().run(builder) {
            ToolState::TestPass
        } else {
            ToolState::TestFail
        };
        builder.save_toolstate(self.name, toolstate);
    }

    /// This runs `rustdoc --test` on all `.md` files in the path.
    fn run_local_doc(self, builder: &Builder<'_>) {
        let compiler = self.compiler;
        let host = self.compiler.host;

        builder.ensure(compile::Std::new(compiler, host));

        let _guard =
            builder.msg(Kind::Test, compiler.stage, format!("book {}", self.name), host, host);

        // Do a breadth-first traversal of the `src/doc` directory and just run
        // tests for all files that end in `*.md`
        let mut stack = vec![builder.src.join(self.path)];
        let _time = helpers::timeit(builder);
        let mut files = Vec::new();
        while let Some(p) = stack.pop() {
            if p.is_dir() {
                stack.extend(t!(p.read_dir()).map(|p| t!(p).path()));
                continue;
            }

            if p.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }

            files.push(p);
        }

        files.sort();

        for file in files {
            markdown_test(builder, compiler, &file);
        }
    }
}

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
                    builder.ensure(BookTest {
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

test_book!(
    Nomicon, "src/doc/nomicon", "nomicon", default=false, submodules=["src/doc/nomicon"];
    Reference, "src/doc/reference", "reference", default=false, submodules=["src/doc/reference"];
    RustdocBook, "src/doc/rustdoc", "rustdoc", default=true;
    RustcBook, "src/doc/rustc", "rustc", default=true;
    RustByExample, "src/doc/rust-by-example", "rust-by-example", default=false, submodules=["src/doc/rust-by-example"];
    EmbeddedBook, "src/doc/embedded-book", "embedded-book", default=false, submodules=["src/doc/embedded-book"];
    TheBook, "src/doc/book", "book", default=false, submodules=["src/doc/book"];
    UnstableBook, "src/doc/unstable-book", "unstable-book", default=true;
    EditionGuide, "src/doc/edition-guide", "edition-guide", default=false, submodules=["src/doc/edition-guide"];
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErrorIndex {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for ErrorIndex {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/tools/error_index_generator")
    }

    fn make_run(run: RunConfig<'_>) {
        // error_index_generator depends on librustdoc. Use the compiler that
        // is normally used to build rustdoc for other tests (like compiletest
        // tests in tests/rustdoc) so that it shares the same artifacts.
        let compiler =
            run.builder.compiler_for(run.builder.top_stage, run.builder.build.build, run.target);
        run.builder.ensure(ErrorIndex {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.builder.build.build,
                compiler,
                target: run.target,
            },
        });
    }

    /// Runs the error index generator tool to execute the tests located in the error
    /// index.
    ///
    /// The `error_index_generator` tool lives in `src/tools` and is used to
    /// generate a markdown file from the error indexes of the code base which is
    /// then passed to `rustdoc --test`.
    fn run(self, builder: &Builder<'_>) {
        let compiler = self.common.compiler;

        let dir = testdir(builder, compiler.host);
        t!(fs::create_dir_all(&dir));
        let output = dir.join("error-index.md");

        let mut tool = tool::ErrorIndex::command(builder);
        tool.arg("markdown").arg(&output);

        let guard =
            builder.msg(Kind::Test, compiler.stage, "error-index", compiler.host, compiler.host);
        let _time = helpers::timeit(builder);
        tool.run_capture(builder);
        drop(guard);
