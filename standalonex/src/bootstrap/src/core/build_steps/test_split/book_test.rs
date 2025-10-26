use std::fs;


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
        // The local docs are testedByDefault, and we don't want to pay the
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
