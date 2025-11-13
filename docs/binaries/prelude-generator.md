## `prelude-generator`

**Purpose:** Generates a prelude for Rust code, processes use statements, and can generate test reports and verification scripts.

**Parameters:**

*   **`--dry-run`**
    *   **Type:** `bool` (flag)
    *   **Description:** Run in dry-run mode, printing changes without modifying files.
*   **`--path <PATH>`**
    *   **Type:** `PathBuf`
    *   **Description:** The path to the workspace root. Defaults to `.`.
*   **`--exclude-crates <EXCLUDE_CRATES>...`**
    *   **Type:** `Vec<String>`
    *   **Description:** Comma-separated list of crate names to exclude from processing.
*   **`--report`**
    *   **Type:** `bool` (flag)
    *   **Description:** Generate a summary report of the prelude generation process. Defaults to `false`.
*   **`--results-file <RESULTS_FILE>`**
    *   **Type:** `PathBuf`
    *   **Description:** Path to a file to save/load processing results. Defaults to `prelude_processing_results.json`.
*   **`--cache-report`**
    *   **Type:** `bool` (flag)
    *   **Description:** Generate a report on the prelude cache. Defaults to `false`.
*   **`--timeout <TIMEOUT>`**
    *   **Type:** `Option<u64>`
    *   **Description:** Timeout in seconds for the prelude generation process.
*   **`--force`**
    *   **Type:** `bool` (flag)
    *   **Description:** Force overwriting of files even if they exist. Defaults to `false`.
*   **`--generate-test-report`**
    *   **Type:** `bool` (flag)
    *   **Description:** Generate a JSON report of all unique test cases found in the repository. Defaults to `false`.
*   **`--test-report-output-file <TEST_REPORT_OUTPUT_FILE>`**
    *   **Type:** `Option<PathBuf>`
    *   **Description:** Path to the output file for the JSON test report. Only used if `generate_test_report` is true.
*   **`--compile-tests`**
    *   **Type:** `bool` (flag)
    **Description:** Generate a test verification script and report from a JSON test report. Defaults to `false`.
*   **`--test-report-input-file <TEST_REPORT_INPUT_FILE>`**
    *   **Type:** `Option<PathBuf>`
    *   **Description:** Path to the JSON test report input file. Required if `compile_tests` is true.
*   **`--test-verification-output-dir <TEST_VERIFICATION_OUTPUT_DIR>`**
    *   **Type:** `Option<PathBuf>`
    *   **Description:** Path to the directory where the test verification script and report will be generated. Required if `compile_tests` is true.
*   **`--extract-use-statements`**
    *   **Type:** `bool` (flag)
    *   **Description:** Extract unique use statements and generate test files for a use statement parser. Defaults to `false`.
*   **`--use-statements-output-dir <USE_STATEMENTS_OUTPUT_DIR>`**
    *   **Type:** `Option<PathBuf>`
    *   **Description:** Path to the directory where generated use statement test files will be placed. Required if `extract_use_statements` is true.
*   **`--collect-and-process-use-statements`**
    *   **Type:** `bool` (flag)
    *   **Description:** Collect and process use statements. Defaults to `false`.
*   **`--generate-aggregated-test-file`**
    *   **Type:** `bool` (flag)
    *   **Description:** Generate a single test file with all unique use statements. Defaults to `false`.
*   **`--run-pipeline`**
    *   **Type:** `bool` (flag)
    *   **Description:** Run the use statement processing pipeline. Defaults to `false`.
*   **`--stage <STAGE>`**
    *   **Type:** `Option<String>`
    *   **Description:** Specify the stage of the pipeline to run.
*   **`--batch-size <BATCH_SIZE>`**
    *   **Type:** `Option<usize>`
    *   **Description:** Process files in batches of this size.
*   **`--batch-limit <BATCH_LIMIT>`**
    *   **Type:** `Option<usize>`
    *   **Description:** The maximum number of batches to run.
*   **`--file <FILE>`**
    *   **Type:** `Option<String>`
    *   **Description:** Process a single file.
*   **`--stop-after <STOP_AFTER>`**
    *   **Type:** `usize`
    *   **Description:** Stop after processing N statements. Defaults to `0`.
*   **`--step-timeout <STEP_TIMEOUT>`**
    *   **Type:** `u64`
    *   **Description:** Timeout in seconds for each processing step. Defaults to `0`.
*   **`-v, --verbose`**
    *   **Type:** `u8` (count)
    *   **Description:** Enable verbose output. Can be specified multiple times for increased verbosity.
