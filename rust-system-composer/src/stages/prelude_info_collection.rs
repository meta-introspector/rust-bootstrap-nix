// rust-system-composer/src/stages/prelude_info_collection.rs

use anyhow::Context; // Re-added
use std::collections::HashMap;
use std::path::Path;
use tokio::runtime::Builder; // Re-added
use crate::cli::{CliArgs, LayeredComposeArgs};
use crate::config::Config;
use crate::config_lock::{StageLock, StageStatus};
use crate::stages::Stage;
use prelude_generator::types::CollectedAnalysisData; // Re-added

pub struct PreludeInfoCollectionStage;

// #[async_trait] // Temporarily remove async_trait
impl Stage for PreludeInfoCollectionStage {
    fn name(&self) -> &str {
        "prelude_info_collection"
    }

    fn run( // Make it synchronous
        &self,
        project_root: &Path,
        config: &Config,
        cli_args: &CliArgs,
        layered_compose_args: &LayeredComposeArgs,
        stage_lock: &mut StageLock,
    ) -> anyhow::Result<()> {
        if layered_compose_args.generate_lock_only {
            println!("GENERATE LOCK ONLY: Skipping prelude info collection and type analysis for stage: {}", self.name());
            // No actual processing, just return Ok. Input hashes will be collected by orchestrator.
            return Ok(());
        }

        println!("Calling prelude-generator::collect_prelude_info to extract constants...");

        let exclude_paths = config.paths.exclude_paths.clone().unwrap_or_default();

        let prelude_generator_args_for_collect_prelude = prelude_generator::Args {
            path: project_root.to_path_buf(), // Search the entire project
            exclude_paths: exclude_paths.clone(), // Use configurable exclusion paths
            verbose: cli_args.verbosity,
            dry_run: layered_compose_args.dry_run,
            ..Default::default()
        };

        // TODO: Add input hashes for prelude_info_collection stage (e.g., source files)

        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()?;

        runtime.block_on(async {
            prelude_generator::collect_prelude_info::collect_prelude_info(
                project_root, // Pass project root as workspace_path
                &prelude_generator_args_for_collect_prelude, // Pass the args with exclusion
            ).await
        })?;

        println!("prelude-generator::collect_prelude_info for constant extraction completed successfully.");

        // Call prelude-generator's type_usage_analyzer::analyze_type_usage directly
        println!("Calling prelude-generator::type_usage_analyzer::analyze_type_usage...");

        let generated_decls_root = config.paths.generated_declarations_root.clone();
        // Ensure the output directory for generated declarations exists
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()?;
        runtime.block_on(async {
            tokio::fs::create_dir_all(&generated_decls_root)
                .await
                .context(format!("Failed to create generated declarations root directory: {}", generated_decls_root.display()))
        })?;

        let _collected_analysis_data: CollectedAnalysisData;
        if layered_compose_args.skip_type_analysis {
            println!("Skipping type analysis stage.");
            _collected_analysis_data = CollectedAnalysisData::default(); // Provide a default or empty instance
        } else {
            // ... (rest of type analysis logic)
        }

        let exclude_paths = config.paths.exclude_paths.clone().unwrap_or_default(); // exclude_paths comes from config.paths

        let prelude_generator_args_for_collect_prelude = prelude_generator::Args {
            path: project_root.to_path_buf(), // Search the entire project
            exclude_paths: exclude_paths, // Use configurable exclusion paths, default to empty Vec if None
            verbose: cli_args.verbosity,
            dry_run: layered_compose_args.dry_run,
            ..Default::default()
        };

        // TODO: Add input hashes for prelude_info_collection stage (e.g., source files)

        let result = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()? 
            .block_on(async {
                prelude_generator::collect_prelude_info::collect_prelude_info(
                    project_root, // Pass project root as workspace_path
                    &prelude_generator_args_for_collect_prelude, // Pass the args with exclusion
                ).await
            });

        match result {
            Ok(_) => {
                println!("prelude-generator::collect_prelude_info for constant extraction completed successfully.");
                stage_lock.status = StageStatus::Executed;
                // TODO: Add output hashes for prelude_info_collection stage (e.g., generated traits/constants)
            }
            Err(e) => {
                eprintln!("Error in prelude info collection: {:?}", e);
                stage_lock.status = StageStatus::Failed;
                return Err(e);
            }
        }
        Ok(())
    }

    fn collect_input_hashes(&self, _project_root: &Path) -> anyhow::Result<HashMap<String, String>> {
        // For now, just return an empty HashMap.
        // TODO: Implement actual input hash collection (e.g., hash of source files).
        Ok(HashMap::new())
    }

    fn collect_output_hashes(&self, _project_root: &Path) -> anyhow::Result<HashMap<String, String>> {
        // For now, just return an empty HashMap.
        // TODO: Implement actual output hash collection (e.g., hash of generated files).
        Ok(HashMap::new())
    }

    fn should_skip(&self, layered_compose_args: &LayeredComposeArgs, _stage_lock: &StageLock) -> bool {
        layered_compose_args.skip_prelude_info
    }

    fn should_force_run(&self, layered_compose_args: &LayeredComposeArgs) -> bool {
        layered_compose_args.force_prelude_info
    }
}
