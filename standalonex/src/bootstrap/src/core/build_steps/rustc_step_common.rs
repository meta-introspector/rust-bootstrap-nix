use crate::builder::ShouldRun;

pub fn rustc_should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
    run.crate_or_deps("rustc-main").path("compiler")
}
