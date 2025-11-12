use crate::prelude::*;
pub mod prelude;
extern crate test_definitions_macro;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Compiler;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TargetSelection;
pub struct ShouldRun;
pub struct RunConfig<'a> {
    pub builder: &'a Builder<'a>,
    pub target: TargetSelection,
}
pub struct Builder<'a> {
    pub top_stage: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
pub mod compiletest {
    use super::{Compiler, TargetSelection};
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Compiletest {
        pub compiler: Compiler,
        pub target: TargetSelection,
        pub mode: &'static str,
        pub suite: &'static str,
        pub path: &'static str,
        pub compare_mode: Option<&'static str>,
    }
}
impl ShouldRun {
    pub fn suite_path(self, _path: &str) -> Self {
        self
    }
}
impl<'a> Builder<'a> {
    pub fn compiler(&self, _stage: u32, _target: TargetSelection) -> Compiler {
        Compiler
    }
    pub fn build_triple(&self) -> TargetSelection {
        TargetSelection
    }
    pub fn ensure<T: Step>(&self, _step: T) {}
}
impl Step for compiletest::Compiletest {
    type Output = ();
    const DEFAULT: bool = false;
    const ONLY_HOSTS: bool = false;
    fn should_run(run: ShouldRun) -> ShouldRun {
        run
    }
    fn make_run(_run: RunConfig) {}
    fn run(self, _builder: &Builder) {}
}
pub trait Step {
    type Output;
    const DEFAULT: bool;
    const ONLY_HOSTS: bool;
    fn should_run(run: ShouldRun) -> ShouldRun;
    fn make_run(run: RunConfig);
    fn run(self, builder: &Builder);
}
test_definitions!(
    MyTest { path : "tests/my_test", mode : "my_mode", suite : "my_suite", default :
    true, host : false, compare_mode : None }
);
fn main() {
    println!("Macro test compiled successfully!");
}
