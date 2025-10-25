use crate::prelude::*


        .lines()
        .find_map(|l| l.split(':').nth(1)?.strip_prefix("browser-ui-test@"))
        .map(|v| v.to_owned())
}

fn get_browser_ui_test_version(builder: &Builder<'_>, npm: &Path) -> Option<String> {
    get_browser_ui_test_version_inner(builder, npm, false)
        .or_else(|| get_browser_ui_test_version_inner(builder, npm, true))
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RustdocGUI {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for RustdocGUI {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let builder = run.builder;
        let run = run.suite_path("tests/rustdoc-gui");
        run.lazy_default_condition(Box::new(move || {
            builder.config.nodejs.is_some()
                && builder.doc_tests != DocTests::Only
                && builder
                    .config
                    .npm
                    .as_ref()
                    .map(|p| get_browser_ui_test_version(builder, p).is_some())
                    .unwrap_or(false)
        }))
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.build_triple());
        run.builder.ensure(RustdocGUI {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.build_triple(),
                compiler,
                target: run.target,
            },
        });
    }

    fn run(self, builder: &Builder<'_>) {
        builder.ensure(compile::Std::new(self.common.compiler, self.common.target));

        let mut cmd = builder.tool_cmd(Tool::RustdocGUITest);

        let out_dir = builder.test_out(self.common.target).join("rustdoc-gui");
        builder.clear_if_dirty(&out_dir, &builder.rustdoc(self.common.compiler));

        if let Some(src) = builder.config.src.to_str() {
            cmd.arg("--rust-src").arg(src);
        }

        if let Some(out_dir) = out_dir.to_str() {
            cmd.arg("--out-dir").arg(out_dir);
