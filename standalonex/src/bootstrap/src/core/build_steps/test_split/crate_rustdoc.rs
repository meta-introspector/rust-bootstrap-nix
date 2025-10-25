use crate::prelude::*


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateRustdoc {
    host: TargetSelection,
}

impl Step for CrateRustdoc {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.paths(&["src/librustdoc", "src/tools/rustdoc"])
    }

    fn make_run(run: RunConfig<'_>) {
        let builder = run.builder;

        builder.ensure(CrateRustdoc { host: run.target });
    }

    fn run(self, builder: &Builder<'_>) {
        let target = self.host;

        let compiler = if builder.download_rustc() {
            builder.compiler(builder.top_stage, target)
        } else {
            // Use the previous stage compiler to reuse the artifacts that are
            // created when running compiletest for tests/rustdoc. If this used
            // `compiler`, then it would cause rustdoc to be built *again*, which
            // isn't really necessary.
            builder.compiler_for(builder.top_stage, target, target)
        };
        builder.ensure(compile::Std::new(compiler, target));
        builder.ensure(compile::Rustc::new(compiler, target));

        let mut cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolRustc,
            target,
            builder.kind,
            "src/tools/rustdoc",
            SourceType::InTree,
            &[],
        );
        if self.host.contains("musl") {
            cargo.arg("-Ctarget-feature=-crt-static");
        }

        let libdir = if builder.download_rustc() {
            builder.rustc_libdir(compiler)
        } else {
            builder.sysroot_target_libdir(compiler, target).to_path_buf()
        };
        let mut dylib_path = dylib_path();
        dylib_path.insert(0, PathBuf::from(&*libdir));
        cargo.env(dylib_path_var(), env::join_paths(&dylib_path).unwrap());

        run_cargo_test(
            cargo,
            &[],
            &["rustdoc:0.0.0".to_string()],
            "rustdoc",
            "rustdoc",
            compiler,
            target,
            builder,
        );
    }
}
