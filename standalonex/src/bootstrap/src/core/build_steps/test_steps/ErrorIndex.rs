use crate::prelude::*


        // The tests themselves need to link to std, so make sure it is
        // available.
        builder.ensure(compile::Std::new(compiler, compiler.host));
        markdown_test(builder, compiler, &output);
    }
}

fn markdown_test(builder: &Builder<'_>, compiler: Compiler, markdown: &Path) -> bool {
    if let Ok(contents) = fs::read_to_string(markdown) {
        if !contents.contains("```") {
            return true;
        }
    }

    builder.verbose(|| println!("doc tests for: {}", markdown.display()));
    let mut cmd = builder.rustdoc_cmd(compiler);
    builder.add_rust_test_threads(&mut cmd);
    // allow for unstable options such as new editions
    cmd.arg("-Z");
    cmd.arg("unstable-options");
    cmd.arg("--test");
    cmd.arg(markdown);
    cmd.env("RUSTC_BOOTSTRAP", "1");

    let test_args = builder.config.test_args().join(" ");
    cmd.arg("--test-args").arg(test_args);

    cmd = cmd.delay_failure();
    if !builder.config.verbose_tests {
        cmd.run_capture(builder).is_success()
    } else {
        cmd.run(builder)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RustcGuide;

impl Step for RustcGuide {
    type Output = ();
    const DEFAULT: bool = false;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/doc/rustc-dev-guide")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(RustcGuide);
    }

    fn run(self, builder: &Builder<'_>) {
        let relative_path = "src/doc/rustc-dev-guide";
        builder.require_submodule(relative_path, None);

        let src = builder.src.join(relative_path);
        let mut rustbook_cmd = builder.tool_cmd(Tool::Rustbook).delay_failure();
        rustbook_cmd.arg("linkcheck").arg(&src);
        let toolstate =
            if rustbook_cmd.run(builder) { ToolState::TestPass } else { ToolState::TestFail };
        builder.save_toolstate("rustc-dev-guide", toolstate);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateLibrustc {
    compiler: Compiler,
    target: TargetSelection,
    crates: Vec<String>,
}

impl Step for CrateLibrustc {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.crate_or_deps("rustc-main").path("compiler")
    }

    fn make_run(run: RunConfig<'_>) {
        let builder = run.builder;
        let host = run.build_triple();
        let compiler = builder.compiler_for(builder.top_stage, host, host);
