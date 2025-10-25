use crate::prelude::*


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateRustdocJsonTypes {
    host: TargetSelection,
}

impl Step for CrateRustdocJsonTypes {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/rustdoc-json-types")
    }

    fn make_run(run: RunConfig<'_>) {
        let builder = run.builder;

        builder.ensure(CrateRustdocJsonTypes { host: run.target });
    }

    fn run(self, builder: &Builder<'_>) {
        let target = self.host;

        // Use the previous stage compiler to reuse the artifacts that are
        // created when running compiletest for tests/rustdoc. If this used
        // `compiler`, then it would cause rustdoc to be built *again*, which
        // isn't really necessary.
        let compiler = builder.compiler_for(builder.top_stage, target, target);
        builder.ensure(compile::Rustc::new(compiler, target));

        let cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolRustc,
            target,
            builder.kind,
            "src/rustdoc-json-types",
            SourceType::InTree,
            &[],
        );

        // FIXME: this looks very wrong, libtest doesn't accept `-C` arguments and the quotes are fishy.
        let libtest_args = if self.host.contains("musl") {
            ["-Ctarget-feature=crt-static"].as_slice()
        } else {
            &[]
        };

        run_cargo_test(
            cargo,
            libtest_args,
            &["rustdoc-json-types".to_string()],
            "rustdoc-json-types",
            "rustdoc-json-types",
            compiler,
            target,
            builder,
        );
    }
}
