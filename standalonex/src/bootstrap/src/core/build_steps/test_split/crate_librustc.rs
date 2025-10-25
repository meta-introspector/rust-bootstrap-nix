use crate::prelude::*


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
        let crates = run.make_run_crates(Alias::Compiler);

        builder.ensure(CrateLibrustc { compiler, target: run.target, crates });
    }

    fn run(self, builder: &Builder<'_>) {
        builder.ensure(compile::Std::new(self.compiler, self.target));

        builder.ensure(Crate {
            compiler: self.compiler,
            target: self.target,
            mode: Mode::Rustc,
            crates: self.crates,
        });
    }
}
