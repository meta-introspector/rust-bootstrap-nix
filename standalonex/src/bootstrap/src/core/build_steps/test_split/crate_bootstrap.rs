use crate::prelude::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateBootstrap {
    path: PathBuf,
    host: TargetSelection,
}

impl Step for CrateBootstrap {
    type Output = ();
    const ONLY_HOSTS: bool = true;
    const DEFAULT: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("src/tools/jsondoclint")
            .path("src/tools/suggest-tests")
            .path("src/tools/replace-version-placeholder")
            .alias("tidyselftest")
    }

    fn make_run(run: RunConfig<'_>) {
        for path in run.paths {
            let path = path.assert_single_path().path.clone();
            run.builder.ensure(CrateBootstrap { host: run.target, path });
        }
    }

    fn run(self, builder: &Builder<'_>) {
        let bootstrap_host = builder.config.build;
        let compiler = builder.compiler(0, bootstrap_host);
        let mut path = self.path.to_str().unwrap();
        if path == "tidyselftest" {
            path = "src/tools/tidy";
        }

        let cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolBootstrap,
            bootstrap_host,
            Kind::Test,
            path,
            SourceType::InTree,
            &[],
        );
        let crate_name = path.rsplit_once('/').unwrap().1;
        run_cargo_test(cargo, &[], &[], crate_name, crate_name, compiler, bootstrap_host, builder);
    }
}
