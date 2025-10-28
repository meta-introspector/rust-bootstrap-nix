use anyhow::Result;
use tempfile::tempdir;
use std::fs;
use std::path::{Path, PathBuf};
use prelude_generator::*;

fn main() -> Result<()> {
    println!("Running generated tests...");
    test_args_default_values()?;
    test_args_custom_values()?;
    test_generate_report_empty_results()?;
    test_generate_report_with_results()?;
    test_generate_prelude_creates_file()?;
    test_generate_prelude_dry_run()?;
    test_generate_prelude_force_overwrite()?;
    test_generate_prelude_no_force_no_overwrite()?;
    test_modify_file_adds_prelude_and_removes_uses()?;
    test_modify_file_dry_run()?;
    test_modify_file_no_use_statements()?;
    test_modify_file_no_force_no_overwrite()?;
    test_modify_file_force_overwrite()?;
    test_process_crates_integration()?;
    test_process_crates_report_only()?;
    test_modify_crate_root_adds_mod_prelude()?;
    test_modify_crate_root_dry_run()?;
    test_modify_crate_root_already_has_prelude()?;
    test_modify_crate_root_main_rs()?;
    test_modify_crate_root_no_force_no_overwrite()?;
    test_modify_crate_root_force_overwrite()?;
    test_extract_test_cases_from_file()?;
    test_collect_all_test_cases()?;
    test_generate_test_report_json()?;
    test_generate_test_runner_crate()?;
    test_generate_aggregated_test_file()?;
    println!("All generated tests passed!");
    Ok(())
}
