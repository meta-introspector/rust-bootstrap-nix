use anyhow::Result;
use tempfile::tempdir;
use std::fs;
use std::path::{Path, PathBuf};
use prelude_generator::*;

fn main() -> Result<()> {
    println!("Running generated tests...");
    prelude_generator::public_tests::test_args_default_values()?;
    prelude_generator::public_tests::test_args_custom_values()?;
    prelude_generator::public_tests::test_generate_report_empty_results()?;
    prelude_generator::public_tests::test_generate_report_with_results()?;
    prelude_generator::public_tests::test_generate_prelude_creates_file()?;
    prelude_generator::public_tests::test_generate_prelude_dry_run()?;
    prelude_generator::public_tests::test_generate_prelude_force_overwrite()?;
    prelude_generator::public_tests::test_generate_prelude_no_force_no_overwrite()?;
    prelude_generator::public_tests::test_modify_file_adds_prelude_and_removes_uses()?;
    prelude_generator::public_tests::test_modify_file_dry_run()?;
    prelude_generator::public_tests::test_modify_file_no_use_statements()?;
    prelude_generator::public_tests::test_modify_file_no_force_no_overwrite()?;
    prelude_generator::public_tests::test_modify_file_force_overwrite()?;
    prelude_generator::public_tests::test_process_crates_integration()?;
    prelude_generator::public_tests::test_process_crates_report_only()?;
    prelude_generator::public_tests::test_modify_crate_root_adds_mod_prelude()?;
    prelude_generator::public_tests::test_modify_crate_root_dry_run()?;
    prelude_generator::public_tests::test_modify_crate_root_already_has_prelude()?;
    prelude_generator::public_tests::test_modify_crate_root_main_rs()?;
    prelude_generator::public_tests::test_modify_crate_root_no_force_no_overwrite()?;
    prelude_generator::public_tests::test_modify_crate_root_force_overwrite()?;
    prelude_generator::public_tests::test_extract_test_cases_from_file()?;
    prelude_generator::public_tests::test_collect_all_test_cases()?;
    prelude_generator::public_tests::test_generate_test_report_json()?;
    prelude_generator::public_tests::test_generate_test_runner_crate()?;
    prelude_generator::public_tests::test_generate_aggregated_test_file()?;
    println!("All generated tests passed!");
    Ok(())
}
