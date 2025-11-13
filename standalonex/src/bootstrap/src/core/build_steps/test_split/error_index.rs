use std::fs;


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
        // The tests themselves need to link to std, so make sure it is
        // available.
        builder.ensure(compile::Std::new(compiler, compiler.host));
        markdown_test(builder, compiler, &output);
    }
}
