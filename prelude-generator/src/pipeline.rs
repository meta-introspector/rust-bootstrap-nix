use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;
use crate::use_extractor::{get_rustc_info, flatten_use_tree, expand_macros_and_parse};
//use tokio::io::AsyncWriteExt;
#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineState {
    pub stage_results: Vec<StageResult>,
    pub processed_files: Vec<String>,
    pub batches_run: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StageResult {
    pub stage_name: String,
    pub summary: StageSummary,
    pub output_file: String,
    pub error_file: Option<String>,
    pub duration_secs: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StageSummary {
    pub total_processed: usize,
    pub successful: usize,
    pub failed: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UseStatement {
    pub statement: String,
    pub error: Option<String>,
}

pub async fn run_pipeline(writer: &mut (impl tokio::io::AsyncWriteExt + Unpin), stage: &Option<String>, batch_size: Option<usize>, batch_limit: Option<usize>, verbose: u8) -> Result<()> {
    let repo_root = PathBuf::from(".");
    let state_file = repo_root.join("generated/pipeline_state.toml");

    let mut state: PipelineState = if state_file.exists() {
        let state_toml = tokio::fs::read_to_string(&state_file).await?;
        toml::from_str(&state_toml)?
    } else {
        PipelineState {
            stage_results: Vec::new(),
            processed_files: Vec::new(),
            batches_run: 0,
        }
    };

    if let Some(limit) = batch_limit {
        if state.batches_run >= limit {
            writer.write_all(format!("Batch limit of {} reached. Stopping.\n", limit).as_bytes()).await?;
            return Ok(());
        }
    }

    let start_time = std::time::Instant::now();
    match stage.as_deref() {
        Some("classify") => {
            writer.write_all(b"Running stage 1: Classify\n").await?;
            run_stage_1_classify(writer, &mut state, batch_size, start_time.elapsed().as_secs_f64(), verbose).await?;
            writer.write_all(format!("Stage 1 finished in {:.2}s\n", start_time.elapsed().as_secs_f64()).as_bytes()).await?;
        }
        Some("preprocess") => {
            writer.write_all(b"Running stage 2: Preprocess\n").await?;
            run_stage_2_preprocess(writer, &mut state, batch_size, start_time.elapsed().as_secs_f64(), verbose).await?;
            writer.write_all(format!("Stage 2 finished in {:.2}s\n", start_time.elapsed().as_secs_f64()).as_bytes()).await?;
        }
        None => {
            writer.write_all(b"Running all stages\n").await?;
            let stage1_start = std::time::Instant::now();
            run_stage_1_classify(writer, &mut state, batch_size, stage1_start.elapsed().as_secs_f64(), verbose).await?;
            let stage1_duration = stage1_start.elapsed().as_secs_f64();
            writer.write_all(format!("Stage 1 finished in {:.2}s\n", stage1_duration).as_bytes()).await?;

            let stage2_start = std::time::Instant::now();
            run_stage_2_preprocess(writer, &mut state, batch_size, stage2_start.elapsed().as_secs_f64(), verbose).await?;
            let stage2_duration = stage2_start.elapsed().as_secs_f64();
            writer.write_all(format!("Stage 2 finished in {:.2}s\n", stage2_duration).as_bytes()).await?;
        }
        Some(stage_name) => anyhow::bail!("Unknown stage: {}", stage_name),
    }

    state.batches_run += 1;
    let updated_state_toml = toml::to_string_pretty(&state)?;
    tokio::fs::write(state_file, updated_state_toml).await?;

    Ok(())
}

async fn run_stage_1_classify(writer: &mut (impl tokio::io::AsyncWriteExt + Unpin), state: &mut PipelineState, batch_size: Option<usize>, duration_secs: f64, verbose: u8) -> Result<()> {
    let repo_root = PathBuf::from(".");

    if verbose > 0 {
        writer.write_all(format!("  [Verbose] Loading all Rust files from repository root...\n").as_bytes()).await?;
    }
    let all_rust_files: Vec<PathBuf> = walkdir::WalkDir::new(&repo_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "rs"))
        .map(|e| e.path().to_path_buf())
        .collect();

    let unprocessed_files: Vec<PathBuf> = all_rust_files
        .into_iter()
        .filter(|f| !state.processed_files.contains(&f.to_string_lossy().to_string()))
        .collect();

    let files_to_process = match batch_size {
        Some(size) => unprocessed_files.into_iter().take(size).collect(),
        None => unprocessed_files,
    };

    if files_to_process.is_empty() {
        writer.write_all(b"No new files to process.\n").await?;
        return Ok(());
    }

    if verbose > 0 {
        writer.write_all(format!("  [Verbose] Found {} files to process.\n", files_to_process.len()).as_bytes()).await?;
    }

    let mut classifications = Vec::new();
    let _rustc_info = get_rustc_info()?;
    let _cache_dir = repo_root.join(".prelude_cache");

    for file_path in &files_to_process {
        if verbose > 1 {
            writer.write_all(format!("    [Verbose] Processing file: {}\n", file_path.display()).as_bytes()).await?;
        }
        let content = tokio::fs::read_to_string(file_path).await?;

        match syn::parse_file(&content) {
            Ok(ast) => {
                if verbose > 2 {
                    writer.write_all(b"      [Verbose] -> Successfully parsed with syn\n").await?;
                }
                for item in ast.items {
                    if let syn::Item::Use(use_item) = item {
                        let mut base_path = Vec::new();
                        flatten_use_tree(&mut base_path, &use_item.tree, &mut classifications);
                    }
                }
            }
            Err(e) => {
                writer.write_all(format!("ERROR: Failed to parse file {}: {}. Attempting macro expansion...\n", file_path.display(), e).as_bytes()).await?;

                let rustc_info = get_rustc_info()?;
                let cache_dir = repo_root.join(".prelude_cache");

                match expand_macros_and_parse(writer, file_path, &content, &rustc_info, &cache_dir).await {
                    Ok(expanded_ast) => {
                        if verbose > 2 {
                            writer.write_all(format!("      [Verbose] -> Successfully expanded macros for: {}\n", file_path.display()).as_bytes()).await?;
                        }
                        // Now try to process the expanded AST
                        for item in expanded_ast.items {
                            if let syn::Item::Use(use_item) = item {
                                let mut base_path = Vec::new();
                                flatten_use_tree(&mut base_path, &use_item.tree, &mut classifications);
                            }
                        }
                    }
                    Err(expand_err) => {
                        writer.write_all(format!("ERROR: Failed to expand macros for file {}: {}\n", file_path.display(), expand_err).as_bytes()).await?;
                        classifications.push(UseStatement {
                            statement: file_path.to_string_lossy().to_string(),
                            error: Some(format!("Syn parse failed: {}; Macro expansion failed: {}", e, expand_err)),
                        });
                    }
                }
            }
        };
    }

    let successful = classifications
        .iter()
        .filter(|c| c.error.is_none())
        .count();
    let failed = classifications.len() - successful;

    let summary = StageSummary {
        total_processed: classifications.len(),
        successful,
        failed,
    };

    let output_file = repo_root.join("generated/stage_1_classify_output.toml");
    let stage_result = StageResult {
        stage_name: "classify".to_string(),
        summary,
        output_file: output_file.to_string_lossy().to_string(),
        error_file: None,
        duration_secs,
    };

    if !classifications.is_empty() {
        let toml_content = toml::to_string_pretty(&classifications)?;
        tokio::fs::write(&output_file, toml_content).await?;
    } else {
        tokio::fs::write(&output_file, "").await?;
    }

    state.stage_results.push(stage_result);
    state.processed_files.extend(files_to_process.into_iter().map(|p| p.to_string_lossy().to_string()));

    writer.write_all(format!("Total use statements collected in Stage 1: {}\n", classifications.len()).as_bytes()).await?;

    Ok(())
}

async fn run_stage_2_preprocess(writer: &mut (impl tokio::io::AsyncWriteExt + Unpin), state: &mut PipelineState, batch_size: Option<usize>, duration_secs: f64, verbose: u8) -> Result<()> {
    let repo_root = PathBuf::from(".");

    let stage_1_output_file = repo_root.join("generated/stage_1_classify_output.toml");
    let toml_content = tokio::fs::read_to_string(stage_1_output_file).await?;
    let classifications: Vec<UseStatement> = toml::from_str(&toml_content)?;

    let syn_error_statements: Vec<_> = classifications
        .into_iter()
        .filter(|c| c.error.is_some())
        .collect();

    let statements_to_process = match batch_size {
        Some(size) => syn_error_statements.into_iter().take(size).collect(),
        None => syn_error_statements,
    };

    if statements_to_process.is_empty() {
        writer.write_all(b"No new SynError statements to process.\n").await?;
        return Ok(());
    }

    if verbose > 0 {
        writer.write_all(format!("  [Verbose] Found {} SynError statements to process.\n", statements_to_process.len()).as_bytes()).await?;
    }

    let mut new_classifications = Vec::new();
    for classification in statements_to_process {
        if let Some(_error) = classification.error {
            if verbose > 1 {
                writer.write_all(format!("    [Verbose] Processing SynError statement: {}\n", classification.statement).as_bytes()).await?;
            }
            let temp_dir = tempfile::tempdir()?;
            let temp_file_path = temp_dir.path().join("main.rs");
            let content = format!("{}\nfn main() {{}}", classification.statement);
            tokio::fs::write(&temp_file_path, content).await?;

            let output = tokio::process::Command::new("rustc")
                .arg(&temp_file_path)
                .output().await?;

            if output.status.success() {
                if verbose > 2 {
                    writer.write_all(b"      [Verbose] -> ParsesWithPreprocessing\n").await?;
                }
                new_classifications.push(UseStatement {
                    statement: classification.statement,
                    error: None,
                });
            } else {
                let error = String::from_utf8_lossy(&output.stderr).to_string();
                if verbose > 2 {
                    writer.write_all(format!("      [Verbose] -> FailsToCompile: {}\n", error).as_bytes()).await?;
                }
                new_classifications.push(UseStatement {
                    statement: classification.statement,
                    error: Some(error),
                });
            }
        }
    }

    let successful = new_classifications
        .iter()
        .filter(|c| c.error.is_none())
        .count();
    let failed = new_classifications.len() - successful;

    let summary = StageSummary {
        total_processed: new_classifications.len(),
        successful,
        failed,
    };

    let output_file = repo_root.join("generated/stage_2_preprocess_output.toml");
    let stage_result = StageResult {
        stage_name: "preprocess".to_string(),
        summary,
        output_file: output_file.to_string_lossy().to_string(),
        error_file: None,
        duration_secs,
    };

    let toml_content = toml::to_string_pretty(&new_classifications)?;
    tokio::fs::write(&output_file, toml_content).await?;

    state.stage_results.push(stage_result);

    Ok(())
}
