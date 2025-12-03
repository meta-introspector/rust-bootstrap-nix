use anyhow::{Context, Result};
use std::boxed::Box;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use crate::prelude_category_pipeline::prelude_category_pipeline_impls::git_project_provisioner::GitProjectProvisioner;
use crate::prelude_category_pipeline::prelude_category_pipeline_impls::hf_validator_invoker::HfValidatorInvoker;
use crate::prelude_category_pipeline::prelude_category_pipeline_impls::utils::copy_dir_all;
use crate::prelude_category_pipeline::prelude_category_pipeline_impls::validation_result_manager::ValidationResultManager;
use crate::args::Args; // Added explicit use
use crate::measurement;
use indoc::indoc;
use pipeline_traits::Config as PipelineConfig;
use pipeline_traits::{ParsedFile, PipelineFunctor, ValidatedFile};
use tempfile::tempdir; // Add this line
                       // HuggingFaceValidatorFunctor
pub struct HuggingFaceValidatorFunctor {
    pub args: Args, // Adjusted
    pub hf_validator_path: Option<PathBuf>,
}

impl PipelineFunctor<ParsedFile, ValidatedFile, PipelineConfig> for HuggingFaceValidatorFunctor {
    fn map<'writer>(
        &'writer self,
        writer: &'writer mut (impl tokio::io::AsyncWriteExt + Unpin + Send),
        input: ParsedFile,
        _config: &'writer Option<PipelineConfig>,
    ) -> Pin<Box<dyn Future<Output = Result<ValidatedFile>> + Send + 'writer>> {
        Box::pin(async move {
            measurement::record_function_entry("HuggingFaceValidatorFunctor::map");
            let ParsedFile(source_code, original_file_path) = input;

            // Declare short_id and generated_output_dir
            let short_id = format!("{:x}", md5::compute(&original_file_path.to_string_lossy().as_bytes()));
            let generated_output_dir = tempdir()?.path().to_path_buf();


            // The source_code is already a String, no need to unparse from AST
            writer
                .write_all(
                    format!("  -> Short ID for hf-validator project: {}\n", short_id).as_bytes(),
                )
                .await?;

            let hf_validator_project_dir = GitProjectProvisioner::provision_project(
                &source_code,
                &original_file_path,
                &generated_output_dir,
            )
            .await?;

            // --- REPLACED CODE BLOCK ---
            let hf_validator_invoker = HfValidatorInvoker {
                hf_validator_path: self.hf_validator_path.clone(),
                args: self.args.clone(), // Clone args for the invoker
            };

            let output_path = hf_validator_invoker
                .invoke_validator(&hf_validator_project_dir, writer)
                .await?;
            // --- END REPLACED CODE BLOCK ---

            let __result = ValidationResultManager::manage_results(
                source_code, // Pass the original source_code
                &output_path,
                &original_file_path,
                &short_id,
                &generated_output_dir,
            )
            .await?;

            measurement::record_function_exit("HuggingFaceValidatorFunctor::map");
            Ok(__result)
        })
    }
}
