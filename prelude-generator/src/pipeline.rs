use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;
use std::fs;
use crate::use_extractor::{get_rustc_info, flatten_use_tree};

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

pub fn run_pipeline(stage: &Option<String>, batch_size: Option<usize>, batch_limit: Option<usize>, verbose: u8) -> Result<()> {
    let repo_root = PathBuf::from(".");
    let state_file = repo_root.join("generated/pipeline_state.toml");

    let mut state: PipelineState = if state_file.exists() {
        let state_toml = fs::read_to_string(&state_file)?;
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
            println!("Batch limit of {} reached. Stopping.", limit);
            return Ok(());
        }
    }

    let start_time = std::time::Instant::now();
    match stage.as_deref() {
        Some("classify") => {
            println!("Running stage 1: Classify");
            run_stage_1_classify(&mut state, batch_size, start_time.elapsed().as_secs_f64(), verbose)?;
            println!("Stage 1 finished in {:.2}s", start_time.elapsed().as_secs_f64());
        }
        Some("preprocess") => {
            println!("Running stage 2: Preprocess");
            run_stage_2_preprocess(&mut state, batch_size, start_time.elapsed().as_secs_f64(), verbose)?;
            println!("Stage 2 finished in {:.2}s", start_time.elapsed().as_secs_f64());
        }
        None => {
            println!("Running all stages");
            let stage1_start = std::time::Instant::now();
            run_stage_1_classify(&mut state, batch_size, stage1_start.elapsed().as_secs_f64(), verbose)?;
            let stage1_duration = stage1_start.elapsed().as_secs_f64();
            println!("Stage 1 finished in {:.2}s", stage1_duration);

            let stage2_start = std::time::Instant::now();
            run_stage_2_preprocess(&mut state, batch_size, stage2_start.elapsed().as_secs_f64(), verbose)?;
            let stage2_duration = stage2_start.elapsed().as_secs_f64();
            println!("Stage 2 finished in {:.2}s", stage2_duration);
        }
        Some(stage_name) => anyhow::bail!("Unknown stage: {}", stage_name),
    }

    state.batches_run += 1;
    let updated_state_toml = toml::to_string_pretty(&state)?;
    fs::write(state_file, updated_state_toml)?;

    Ok(())
}

fn run_stage_1_classify(state: &mut PipelineState, batch_size: Option<usize>, duration_secs: f64, verbose: u8) -> Result<()> {
    let repo_root = PathBuf::from(".");

    if verbose > 0 {
        println!("  [Verbose] Loading all Rust files from repository root...");
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
        println!("No new files to process.");
        return Ok(());
    }

    if verbose > 0 {
        println!("  [Verbose] Found {} files to process.", files_to_process.len());
    }

    let mut classifications = Vec::new();
    let _rustc_info = get_rustc_info()?;
    let _cache_dir = repo_root.join(".prelude_cache");

    for file_path in &files_to_process {
        if verbose > 1 {
            println!("    [Verbose] Processing file: {}", file_path.display());
        }
        let content = fs::read_to_string(file_path)?;

        match syn::parse_file(&content) {
            Ok(ast) => {
                if verbose > 2 {
                    println!("      [Verbose] -> Successfully parsed with syn");
                }
                for item in ast.items {
                    if let syn::Item::Use(use_item) = item {
                        let mut base_path = Vec::new();
                        flatten_use_tree(&mut base_path, &use_item.tree, &mut classifications);
                    }
                }
            }
            Err(e) => {
                if verbose > 2 {
                    println!("      [Verbose] -> Failed to parse with syn: {}", e);
                }
                // We can't parse the file, so we can't extract use statements yet.
                // We'll just classify the whole file as a syn error for now.
                // In a later stage, we can try to extract use statements after macro expansion.
                classifications.push(UseStatement {
                    statement: file_path.to_string_lossy().to_string(),
                    error: Some(e.to_string()),
                });
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
        fs::write(&output_file, toml_content)?;
    } else {
        fs::write(&output_file, "")?;
    }

    state.stage_results.push(stage_result);
    state.processed_files.extend(files_to_process.into_iter().map(|p| p.to_string_lossy().to_string()));

    Ok(())
}

fn run_stage_2_preprocess(state: &mut PipelineState, batch_size: Option<usize>, duration_secs: f64, verbose: u8) -> Result<()> {
    let repo_root = PathBuf::from(".");

    let stage_1_output_file = repo_root.join("generated/stage_1_classify_output.toml");
    let toml_content = fs::read_to_string(stage_1_output_file)?;
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
        println!("No new SynError statements to process.");
        return Ok(());
    }

    if verbose > 0 {
        println!("  [Verbose] Found {} SynError statements to process.", statements_to_process.len());
    }

    let mut new_classifications = Vec::new();
    for classification in statements_to_process {
        if let Some(_error) = classification.error {
            if verbose > 1 {
                println!("    [Verbose] Processing SynError statement: {}", classification.statement);
            }
            let temp_dir = tempfile::tempdir()?;
            let temp_file_path = temp_dir.path().join("main.rs");
            let content = format!("{}\nfn main() {{}}", classification.statement);
            fs::write(&temp_file_path, content)?;

            let output = std::process::Command::new("rustc")
                .arg(&temp_file_path)
                .output()?;

            if output.status.success() {
                if verbose > 2 {
                    println!("      [Verbose] -> ParsesWithPreprocessing");
                }
                new_classifications.push(UseStatement {
                    statement: classification.statement,
                    error: None,
                });
            } else {
                let error = String::from_utf8_lossy(&output.stderr).to_string();
                if verbose > 2 {
                    println!("      [Verbose] -> FailsToCompile: {}", error);
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
    fs::write(&output_file, toml_content)?;

    state.stage_results.push(stage_result);

    Ok(())
}
