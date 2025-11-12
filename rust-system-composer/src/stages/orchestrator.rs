use anyhow::Result;
use std::path::Path;
use crate::cli::{CliArgs, LayeredComposeArgs};
use crate::config::Config;
use crate::config_lock::{ConfigLock, StageStatus}; // StageLock is not directly used here
use crate::stages::Stage;
use crate::stages::prelude_info_collection::PreludeInfoCollectionStage;

pub struct StageOrchestrator {
    stages: Vec<Box<dyn Stage>>,
}

impl StageOrchestrator {
    pub fn new() -> Self {
        let mut stages: Vec<Box<dyn Stage>> = Vec::new();
        stages.push(Box::new(PreludeInfoCollectionStage));
        // Add other stages here as they are implemented
        StageOrchestrator { stages }
    }

    pub fn run_stages(
        &self,
        project_root: &Path,
        config: &Config,
        cli_args: &CliArgs,
        layered_compose_args: &LayeredComposeArgs,
        config_lock: &mut ConfigLock,
    ) -> Result<()> {
        for stage in &self.stages {
            println!("Running stage: {}", stage.name());

            let mut stage_lock = config_lock.get_or_create_stage_lock(stage.name());

            // TODO: Implement caching/skipping logic here based on stage_lock and layered_compose_args

            if layered_compose_args.generate_lock_only {
                println!("GENERATE LOCK ONLY: Collecting input hashes for stage: {}", stage.name());
                stage_lock.input_hashes = stage.collect_input_hashes(project_root)?;
                stage_lock.status = StageStatus::Skipped; // Mark as skipped since it's not fully executed
            } else {
                // Execute the stage
                match stage.run(
                    project_root,
                    config,
                    cli_args,
                    layered_compose_args,
                    &mut stage_lock,
                ) {
                    Ok(_) => {
                        println!("Stage {} completed successfully.", stage.name());
                        stage_lock.status = StageStatus::Executed;
                    }
                    Err(e) => {
                        eprintln!("Stage {} failed with error: {:?}", stage.name(), e);
                        stage_lock.status = StageStatus::Failed;
                        return Err(e);
                    }
                }
            }
            config_lock.update_stage_lock(stage_lock);
        }
        Ok(())
    }
}
