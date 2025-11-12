use build_helper::prelude::*;
#[derive(Clone, Debug, Default)]
pub enum RustfmtState {
    SystemToolchain(PathBuf),
    Downloaded(PathBuf),
    Unavailable,
    #[default]
    LazyEvaluated,
}
