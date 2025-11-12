use std::{
    env, fs, io,
    path::{PathBuf},
    process::{Command, Stdio},
};
use tempfile::tempdir;
use syn;
use quote::ToTokens;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <wrapped_code_path> <rollup_data_dir>", args[0]);
        return Ok(())
    }

    let wrapped_code_path = PathBuf::from(&args[1]);
    let rollup_data_dir = PathBuf::from(&args[2]);

    if !wrapped_code_path.is_file() {
        eprintln!("Error: Wrapped code file does not exist: {}", wrapped_code_path.display());
        return Ok(())
    }
    if !rollup_data_dir.is_dir() {
        eprintln!("Error: Rollup data directory does not exist: {}", rollup_data_dir.display());
        return Ok(())
    }

    // 1. Create a temporary Rust project
    let temp_dir = tempdir()?;
    let temp_project_path = temp_dir.path().join("temp_rust_project");
    Command::new("cargo")
        .arg("new")
        .arg("--bin")
        .arg(&temp_project_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    // 2. Copy measurement.rs into this temporary project's src directory
    let temp_src_path = temp_project_path.join("src");

    // Assuming measurement.rs is in the same directory as wrapped_code.rs for now,
    // or we need a way to locate it. For this PoC, let's assume it's provided.
    // In a real scenario, measurement.rs would be a dependency or part of a common library.
    // For now, we'll copy it from rust-decl-splitter's src.
    let measurement_rs_path = PathBuf::from("/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix/rust-decl-splitter/src/measurement.rs");
    fs::copy(&measurement_rs_path, temp_src_path.join("measurement.rs"))?;

    let _function_name = wrapped_code_path
        .parent() // Get the parent directory (rollup_data)
        .unwrap()
        .parent() // Get the parent directory (my_function_one)
        .unwrap()
        .file_name() // Get the file name (my_function_one)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let temp_wrapped_code_file_name = wrapped_code_path.file_name().unwrap(); // This is "wrapped_code.rs"
    let temp_wrapped_code_dest_path = temp_src_path.join(temp_wrapped_code_file_name);

    fs::copy(
        &wrapped_code_path,
        &temp_wrapped_code_dest_path, // Copy to wrapped_code.rs
    )?;

    let mut wrapped_code_content = fs::read_to_string(&temp_wrapped_code_dest_path)?;
    wrapped_code_content = wrapped_code_content.replace("use prelude::*;", ""); // Remove use prelude::*;
    wrapped_code_content = wrapped_code_content.replace("use crate::measurement;", ""); // Remove use crate::measurement;
    fs::write(&temp_wrapped_code_dest_path, wrapped_code_content.clone())?;

    // 3. Modify Cargo.toml of the temporary project
    let temp_cargo_toml_path = temp_project_path.join("Cargo.toml");
    let mut cargo_toml_content = fs::read_to_string(&temp_cargo_toml_path)?;
    cargo_toml_content = cargo_toml_content.replace(
        "[dependencies]",
        "[dependencies]\nserde = { version = \"1.0\", features = [\"derive\"] }\nserde_json = \"1.0\"
lazy_static = \"1.4.0\"",
    );
    fs::write(&temp_cargo_toml_path, cargo_toml_content)?;

    // 4. Create a main.rs in the temporary project that calls the function and prints JSON metrics
    let function_name = wrapped_code_path
        .parent() // Get the parent directory (rollup_data)
        .unwrap()
        .parent() // Get the parent directory (my_function_one)
        .unwrap()
        .file_name() // Get the file name (my_function_one)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let temp_main_rs_content = format!(
        r#"
mod measurement; // Re-add mod measurement;
{} // Directly include the wrapped_code content here

fn main() {{
    let result = {}(); // Call the function directly with no arguments
    println!("Function call result: {{}}", result);

    let collected_metrics = measurement::get_collected_metrics();
    let json_metrics = serde_json::to_string_pretty(&collected_metrics).expect("Failed to serialize metrics to JSON");
    println!("--- METRICS_START ---\n{{}}\n--- METRICS_END ---", json_metrics);
}}
"#,
        wrapped_code_content, // Use the content that was already read and modified
        function_name
    );
    fs::write(temp_src_path.join("main.rs"), temp_main_rs_content)?;

    // 5. Build and run this temporary project
    let output = Command::new("cargo")
        .arg("run")
        .current_dir(&temp_project_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        eprintln!(
            "Error running temporary project:\nStdout: {}\nStderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        return Ok(())
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    // 6. Capture the stdout, extract JSON metrics, and save to rollup_report.md
    let metrics_start_tag = "--- METRICS_START ---";
    let metrics_end_tag = "--- METRICS_END ---";

    let mut rollup_report_content = String::new();
    rollup_report_content.push_str(&format!("# Rollup Report for Function: {}\n\n", function_name));
    rollup_report_content.push_str(&format!("## Original Code (`{}`):\n", wrapped_code_path.display()));
    rollup_report_content.push_str("```rust\n");
    rollup_report_content.push_str(&wrapped_code_content); // Use the cleaned wrapped_code_content
    rollup_report_content.push_str("\n```\n\n");

    // Calculate line count
    let line_count = wrapped_code_content.lines().count();
    rollup_report_content.push_str("## Code Metrics:\n");
    rollup_report_content.push_str(&format!("*   **Line Count:** {}\n\n", line_count));

    // Placeholder for Use Statement Analysis
    let mut total_use_statements = 0;
    let mut std_uses = 0;
    let mut crate_uses = 0;
    let mut external_uses = 0;

    match syn::parse_file(&wrapped_code_content) {
        Ok(ast) => {
            for item in ast.items {
                if let syn::Item::Use(use_item) = item {
                    total_use_statements += 1;
                    let use_path = use_item.tree.to_token_stream().to_string();
                    if use_path.starts_with("std::") {
                        std_uses += 1;
                    } else if use_path.starts_with("crate::") {
                        crate_uses += 1;
                    } else {
                        external_uses += 1;
                    }
                }
            }
        }
        Err(e) => {
            // Handle parsing error, maybe log it or include in the report
            eprintln!("Error parsing wrapped code for use statement analysis: {}", e);
        }
    }

    rollup_report_content.push_str("## Use Statement Analysis:\n");
    rollup_report_content.push_str(&format!("*   **Total Use Statements Processed:** {}\n", total_use_statements));
    rollup_report_content.push_str("*   **Types of Use Statements:**\n");
    rollup_report_content.push_str(&format!("    *   `std::`: {}\n", std_uses));
    rollup_report_content.push_str(&format!("    *   `crate::`: {}\n", crate_uses));
    rollup_report_content.push_str(&format!("    *   External Crates: {}\n\n", external_uses));

    if let Some(start_index) = stdout.find(metrics_start_tag) {
        if let Some(end_index) = stdout.find(metrics_end_tag) {
            let json_str = &stdout[start_index + metrics_start_tag.len()..end_index].trim();
            rollup_report_content.push_str("## Performance Metrics:\n");
            rollup_report_content.push_str("```json\n");
            rollup_report_content.push_str(json_str);
            rollup_report_content.push_str("\n```\n");
        }
    }

    fs::write(rollup_data_dir.join("rollup_report.md"), rollup_report_content)?;
    println!("Successfully generated rollup_report.md for {}", wrapped_code_path.display());

    Ok(())
}