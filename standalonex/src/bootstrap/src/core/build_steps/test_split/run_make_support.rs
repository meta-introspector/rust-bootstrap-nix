use crate::prelude::*;


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct RunMakeSupport {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for RunMakeSupport {
    type Output = PathBuf;
    const DEFAULT: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.build_triple());
        run.builder.ensure(RunMakeSupport {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.build_triple(),
                compiler,
                target: run.build_triple(),
            },
        });
    }

    /// Builds run-make-support and returns the path to the resulting rlib.
    fn run(self, builder: &Builder<'_>) -> PathBuf {
        builder.ensure(compile::Std::new(self.common.compiler, self.common.target));

        let cargo = tool::prepare_tool_cargo(
            builder,
            self.common.compiler,
            Mode::ToolStd,
            self.common.target,
            Kind::Build,
            "src/tools/run-make-support",
            SourceType::InTree,
            &[],
        );

        cargo.into_cmd().run(builder);

        let lib_name = "librun_make_support.rlib";
        let lib = builder.tools_dir(self.common.compiler).join(lib_name);

        let cargo_out = builder.cargo_out(self.common.compiler, Mode::ToolStd, self.common.target).join(lib_name);
        builder.copy_link(&cargo_out, &lib);
        lib
    }
}
