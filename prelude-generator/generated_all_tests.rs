use anyhow::Result;
use std::path::PathBuf;
#[cfg(test)]
mod generated_tests {
    #[test]
    fn test_args_default_values() {
        let args = Args::parse_from(&["prelude-generator"]);
        assert!(! args.dry_run);
        assert_eq!(args.path, PathBuf::from("."));
        assert!(args.exclude_crates.is_empty());
        assert!(! args.report);
        assert_eq!(args.results_file, PathBuf::from("prelude_processing_results.json"));
        assert!(! args.cache_report);
        assert!(args.timeout.is_none());
        assert!(! args.force);
    }
    #[test]
    fn test_args_custom_values() {
        let args = Args::parse_from(
            &[
                "prelude-generator",
                "--dry-run",
                "--path",
                "/tmp/my_project",
                "--exclude-crates",
                "crate1,crate2",
                "--report",
                "--results-file",
                "custom_results.json",
                "--cache-report",
                "--timeout",
                "60",
                "--force",
            ],
        );
        assert!(args.dry_run);
        assert_eq!(args.path, PathBuf::from("/tmp/my_project"));
        assert_eq!(
            args.exclude_crates, vec!["crate1".to_string(), "crate2".to_string()]
        );
        assert!(args.report);
        assert_eq!(args.results_file, PathBuf::from("custom_results.json"));
        assert!(args.cache_report);
        assert_eq!(args.timeout, Some(60));
        assert!(args.force);
    }
    #[test]
    fn test_generate_report_empty_results() -> Result<()> {
        let dir = tempdir()?;
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(&dir)?;
        generate_report(&[])?;
        let report_path = dir.path().join("prelude_generator_summary.md");
        assert!(report_path.exists());
        let content = fs::read_to_string(&report_path)?;
        assert!(content.contains("# Prelude Generation Summary Report"));
        assert!(content.contains("- Total files processed: 0"));
        assert!(! content.contains("## Detailed Results"));
        std::env::set_current_dir(&original_dir)?;
        Ok(())
    }
    #[test]
    fn test_generate_report_with_results() -> Result<()> {
        let dir = tempdir()?;
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(&dir)?;
        let results = vec![
            FileProcessingResult { path : PathBuf::from("src/file1.rs"), status :
            FileProcessingStatus::Success, }, FileProcessingResult { path :
            PathBuf::from("src/file2.rs"), status : FileProcessingStatus::Skipped {
            reason : "already processed".to_string() }, }, FileProcessingResult { path :
            PathBuf::from("src/file3.rs"), status : FileProcessingStatus::Failed { error
            : "syntax error".to_string() }, },
        ];
        generate_report(&results)?;
        let report_path = dir.path().join("prelude_generator_summary.md");
        assert!(report_path.exists());
        let content = fs::read_to_string(&report_path)?;
        assert!(content.contains("# Prelude Generation Summary Report"));
        assert!(content.contains("- Total files processed: 3"));
        assert!(content.contains("- Successfully processed: 1"));
        assert!(content.contains("- Skipped: 1"));
        assert!(content.contains("- Failed: 1"));
        assert!(
            content.contains("### src/file1.rs\n- Status: ✅ Successfully Processed")
        );
        assert!(
            content
            .contains("### src/file2.rs\n- Status: ⏭️ Skipped (Reason: already processed")
        );
        assert!(
            content
            .contains("### src/file3.rs\n- Status: ❌ Failed (Error: syntax error")
        );
        std::env::set_current_dir(&original_dir)?;
        Ok(())
    }
    #[test]
    fn test_generate_prelude_creates_file() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;
        let prelude_content = "// Test prelude content";
        generate_prelude(&src_dir, prelude_content, false, false)?;
        let prelude_path = src_dir.join("prelude.rs");
        assert!(prelude_path.exists());
        assert_eq!(fs::read_to_string(& prelude_path) ?, prelude_content);
        Ok(())
    }
    #[test]
    fn test_generate_prelude_dry_run() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;
        let prelude_content = "// Test prelude content";
        generate_prelude(&src_dir, prelude_content, true, false)?;
        let prelude_path = src_dir.join("prelude.rs");
        assert!(! prelude_path.exists());
        Ok(())
    }
    #[test]
    fn test_generate_prelude_force_overwrite() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;
        let prelude_path = src_dir.join("prelude.rs");
        fs::write(&prelude_path, "// Original content")?;
        let new_prelude_content = "// New prelude content";
        generate_prelude(&src_dir, new_prelude_content, false, true)?;
        assert!(prelude_path.exists());
        assert_eq!(fs::read_to_string(& prelude_path) ?, new_prelude_content);
        Ok(())
    }
    #[test]
    fn test_generate_prelude_no_force_no_overwrite() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;
        let prelude_path = src_dir.join("prelude.rs");
        let original_content = "// Original content";
        fs::write(&prelude_path, original_content)?;
        let new_prelude_content = "// New prelude content";
        generate_prelude(&src_dir, new_prelude_content, false, false)?;
        assert!(prelude_path.exists());
        assert_eq!(fs::read_to_string(& prelude_path) ?, original_content);
        Ok(())
    }
    #[test]
    fn test_modify_file_adds_prelude_and_removes_uses() -> Result<()> {
        let dir = tempdir()?;
        let file_path = setup_test_file(
            &dir,
            "test_file.rs",
            "use std::collections::HashMap;\nuse crate::another_module;\n\nfn main() {}\n",
        );
        modify_file(&file_path, false, true)?;
        let content = fs::read_to_string(&file_path)?;
        assert!(content.contains("use crate::prelude::*;"));
        assert!(! content.contains("use std::collections::HashMap;"));
        assert!(! content.contains("use crate::another_module;"));
        assert!(content.contains("fn main() {}"));
        Ok(())
    }
    #[test]
    fn test_modify_file_dry_run() -> Result<()> {
        let dir = tempdir()?;
        let file_path = setup_test_file(
            &dir,
            "test_file.rs",
            "use std::collections::HashMap;\nfn main() {}\n",
        );
        let original_content = fs::read_to_string(&file_path)?;
        modify_file(&file_path, true, false)?;
        let content = fs::read_to_string(&file_path)?;
        assert_eq!(content, original_content);
        Ok(())
    }
    #[test]
    fn test_modify_file_no_use_statements() -> Result<()> {
        let dir = tempdir()?;
        let file_path = setup_test_file(&dir, "test_file.rs", "fn main() {}\n");
        let original_content = fs::read_to_string(&file_path)?;
        modify_file(&file_path, false, false)?;
        let content = fs::read_to_string(&file_path)?;
        assert_eq!(content, original_content);
        Ok(())
    }
    #[test]
    fn test_modify_file_no_force_no_overwrite() -> Result<()> {
        let dir = tempdir()?;
        let file_path = setup_test_file(
            &dir,
            "test_file.rs",
            "use std::fmt;\nfn some_func() {}\n",
        );
        let original_content = fs::read_to_string(&file_path)?;
        modify_file(&file_path, false, false)?;
        let content_after_skip = fs::read_to_string(&file_path)?;
        assert_eq!(original_content, content_after_skip);
        Ok(())
    }
    #[test]
    fn test_modify_file_force_overwrite() -> Result<()> {
        let dir = tempdir()?;
        let file_path = setup_test_file(
            &dir,
            "test_file.rs",
            "use std::fmt;\nfn some_func() {}\n",
        );
        modify_file(&file_path, false, true)?;
        let first_modified_content = fs::read_to_string(&file_path)?;
        assert!(first_modified_content.contains("use crate::prelude::*;"));
        assert!(! first_modified_content.contains("use std::fmt;"));
        modify_file(&file_path, false, true)?;
        let second_modified_content = fs::read_to_string(&file_path)?;
        assert_eq!(first_modified_content, second_modified_content);
        Ok(())
    }
    #[test]
    fn test_process_crates_integration() -> Result<()> {
        let temp_dir = tempdir()?;
        let project_root = temp_dir.path().to_path_buf();
        let crate1_path = setup_test_crate(
            &project_root,
            "my-crate",
            "use std::collections::HashMap;\nfn my_func() {}\n",
        );
        let args = Args {
            dry_run: false,
            path: project_root.clone(),
            exclude_crates: vec![],
            report: false,
            results_file: project_root.join("results.json"),
            cache_report: false,
            timeout: None,
            force: true,
        };
        process_crates(&args)?;
        let prelude_path = crate1_path.join("src/prelude.rs");
        assert!(prelude_path.exists());
        assert!(
            fs::read_to_string(& prelude_path)
            ?.contains("// This is a generated prelude file")
        );
        let lib_rs_path = crate1_path.join("src/lib.rs");
        let lib_rs_content = fs::read_to_string(&lib_rs_path)?;
        assert!(lib_rs_content.contains("use crate::prelude::*;"));
        assert!(! lib_rs_content.contains("use std::collections::HashMap;"));
        let results_file_content = fs::read_to_string(&args.results_file)?;
        let results: Vec<FileProcessingResult> = serde_json::from_str(
            &results_file_content,
        )?;
        assert_eq!(results.len(), 2);
        assert!(
            results.iter().any(| r | r.path.ends_with("src/lib.rs") && matches!(r.status,
            FileProcessingStatus::Success))
        );
        assert!(
            results.iter().any(| r | r.path.ends_with("src/prelude.rs") && matches!(r
            .status, FileProcessingStatus::Success))
        );
        Ok(())
    }
    #[test]
    fn test_process_crates_report_only() -> Result<()> {
        let temp_dir = tempdir()?;
        let project_root = temp_dir.path().to_path_buf();
        let dummy_results = vec![
            FileProcessingResult { path : PathBuf::from("dummy/file.rs"), status :
            FileProcessingStatus::Success, },
        ];
        let results_json_path = project_root.join("dummy_results.json");
        fs::write(&results_json_path, serde_json::to_string_pretty(&dummy_results)?)?;
        let args = Args {
            dry_run: false,
            path: project_root.clone(),
            exclude_crates: vec![],
            report: true,
            results_file: results_json_path.clone(),
            cache_report: false,
            timeout: None,
            force: false,
        };
        process_crates(&args)?;
        let report_path = project_root.join("prelude_generator_summary.md");
        assert!(report_path.exists());
        let report_content = fs::read_to_string(&report_path)?;
        assert!(report_content.contains("Prelude Generation Summary Report"));
        assert!(report_content.contains("dummy/file.rs"));
        Ok(())
    }
    #[test]
    fn test_modify_crate_root_adds_mod_prelude() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;
        let lib_rs_path = setup_test_file(&dir, "src/lib.rs", "fn main() {}\n");
        modify_crate_root(&src_dir, false, true)?;
        let content = fs::read_to_string(&lib_rs_path)?;
        assert!(content.contains("pub mod prelude;"));
        Ok(())
    }
    #[test]
    fn test_modify_crate_root_dry_run() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;
        let lib_rs_path = setup_test_file(&dir, "src/lib.rs", "fn main() {}\n");
        let original_content = fs::read_to_string(&lib_rs_path)?;
        modify_crate_root(&src_dir, true, false)?;
        let content = fs::read_to_string(&lib_rs_path)?;
        assert_eq!(content, original_content);
        Ok(())
    }
    #[test]
    fn test_modify_crate_root_already_has_prelude() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;
        let lib_rs_path = setup_test_file(
            &dir,
            "src/lib.rs",
            "pub mod prelude;\nfn main() {}\n",
        );
        let original_content = fs::read_to_string(&lib_rs_path)?;
        modify_crate_root(&src_dir, false, false)?;
        let content = fs::read_to_string(&lib_rs_path)?;
        assert_eq!(content, original_content);
        Ok(())
    }
    #[test]
    fn test_modify_crate_root_main_rs() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;
        let main_rs_path = setup_test_file(&dir, "src/main.rs", "fn main() {}\n");
        modify_crate_root(&src_dir, false, true)?;
        let content = fs::read_to_string(&main_rs_path)?;
        assert!(content.contains("pub mod prelude;"));
        Ok(())
    }
    #[test]
    fn test_modify_crate_root_no_force_no_overwrite() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;
        let lib_rs_path = setup_test_file(&dir, "src/lib.rs", "fn main() {}\n");
        modify_crate_root(&src_dir, false, true)?;
        let first_modified_content = fs::read_to_string(&lib_rs_path)?;
        assert!(first_modified_content.contains("pub mod prelude;"));
        modify_crate_root(&src_dir, false, false)?;
        let content_after_skip = fs::read_to_string(&lib_rs_path)?;
        assert_eq!(first_modified_content, content_after_skip);
        Ok(())
    }
    #[test]
    fn test_modify_crate_root_force_overwrite() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;
        let lib_rs_path = setup_test_file(&dir, "src/lib.rs", "fn main() {}\n");
        modify_crate_root(&src_dir, false, true)?;
        let first_modified_content = fs::read_to_string(&lib_rs_path)?;
        assert!(first_modified_content.contains("pub mod prelude;"));
        modify_crate_root(&src_dir, false, true)?;
        let second_modified_content = fs::read_to_string(&lib_rs_path)?;
        assert_eq!(first_modified_content, second_modified_content);
        Ok(())
    }
    #[test]
    fn test_extract_test_cases_from_file() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test_file.rs");
        fs::write(
            &file_path,
            r#"
            #[test]
            fn my_test_1() { assert_eq!(1, 1); }

            fn not_a_test() { }

            #[cfg(test)]
            mod my_tests {
                #[test]
                fn nested_test() { assert_eq!(2, 2); }
            }

            #[test]
            async fn my_test_2() -> Result<()> { Ok(()) }
        "#,
        )?;
        let tests = extract_test_cases_from_file(&file_path)?;
        assert_eq!(tests.len(), 3);
        let test_names: HashSet<String> = tests
            .into_iter()
            .map(|f| f.sig.ident.to_string())
            .collect();
        assert!(test_names.contains("my_test_1"));
        assert!(test_names.contains("my_test_2"));
        assert!(test_names.contains("nested_test"));
        Ok(())
    }
    #[test]
    fn test_collect_all_test_cases() -> Result<()> {
        let dir = tempdir()?;
        let crate_root = dir.path().to_path_buf();
        fs::create_dir_all(crate_root.join("src"))?;
        fs::create_dir_all(crate_root.join("tests"))?;
        fs::write(
            crate_root.join("src/lib.rs"),
            r#"
            #[test]
            fn lib_test() { }
            #[cfg(test)]
            mod lib_tests {
                #[test]
                fn nested_lib_test() { }
            }
        "#,
        )?;
        fs::write(
            crate_root.join("tests/integration_test.rs"),
            r#"
            #[test]
            fn integration_test() { }
        "#,
        )?;
        fs::write(
            crate_root.join("src/another_mod.rs"),
            r#"
            #[test]
            fn another_mod_test() { }
        "#,
        )?;
        let tests = collect_all_test_cases(&crate_root)?;
        assert_eq!(tests.len(), 4);
        let test_names: HashSet<String> = tests
            .into_iter()
            .map(|f| f.sig.ident.to_string())
            .collect();
        assert!(test_names.contains("lib_test"));
        assert!(test_names.contains("nested_lib_test"));
        assert!(test_names.contains("integration_test"));
        assert!(test_names.contains("another_mod_test"));
        Ok(())
    }
    #[test]
    fn test_generate_aggregated_test_file() -> Result<()> {
        let dir = tempdir()?;
        let output_path = dir.path().join("aggregated_tests.rs");
        let test_func1: ItemFn = syn::parse_quote! {
            #[test] fn generated_test_1() { assert_eq!(2, 2); }
        };
        let test_func2: ItemFn = syn::parse_quote! {
            #[test] async fn generated_test_2() -> Result < () > { Ok(()) }
        };
        generate_aggregated_test_file(
            output_path.as_path(),
            vec![test_func1, test_func2],
        )?;
        let content = fs::read_to_string(&output_path)?;
        assert!(content.contains("#[cfg(test)]"));
        assert!(content.contains("mod generated_tests {"));
        assert!(content.contains("fn generated_test_1()"));
        assert!(content.contains("async fn generated_test_2()"));
        assert!(content.contains("use anyhow::Result;"));
        Ok(())
    }
}
