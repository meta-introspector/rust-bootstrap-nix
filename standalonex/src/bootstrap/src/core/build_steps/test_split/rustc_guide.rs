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
