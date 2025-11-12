// rust-system-composer/src/stages/mod.rs

use std::collections::HashMap;
use std::path::Path;
use crate::cli::{CliArgs, LayeredComposeArgs};
use crate::config::Config;
use crate::config_lock::{StageLock, StageStatus};

// #[async_trait] // Temporarily remove async_trait
pub trait Stage {
    /// The name of the stage.
    fn name(&self) -> &str;

    /// Executes the logic for this stage.
    ///
    /// Returns an updated StageLock with execution details, or an error.
    fn run( // Make it synchronous
        &self,
        project_root: &Path,
        config: &Config,
        cli_args: &CliArgs,
        layered_compose_args: &LayeredComposeArgs,
        stage_lock: &mut StageLock,
    ) -> anyhow::Result<()>;

    fn collect_input_hashes(&self, project_root: &Path) -> anyhow::Result<HashMap<String, String>>;

    fn collect_output_hashes(&self, project_root: &Path) -> anyhow::Result<HashMap<String, String>>;

    fn should_skip(&self, _layered_compose_args: &LayeredComposeArgs, _stage_lock: &StageLock) -> bool {
        _layered_compose_args.skip_prelude_info // Placeholder, needs to be dynamic
    }

    fn should_force_run(&self, _layered_compose_args: &LayeredComposeArgs) -> bool {
        _layered_compose_args.force_prelude_info // Placeholder, needs to be dynamic
    }
}

pub mod prelude_info_collection;
pub mod orchestrator; // Declare the prelude_info_collection module
