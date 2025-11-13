use anyhow::Result;
use std::fs;
use std::path::Path;
use syn::Item;
use prettyplease;

/// Modifies a source file to remove its `use` statements and add `use crate::prelude::*;`.
pub fn modify_file(path: &Path, dry_run: bool, force: bool) -> Result<()> {
    println!("  -> Entering modify_file for path: {}", path.display());
    let content = fs::read_to_string(path)?;
    let ast = syn::parse_file(&content)?;
    let mut new_items = Vec::new();
    let mut has_use_statements = false;

    for item in &ast.items {
        if let Item::Use(_) = item {
            has_use_statements = true;
        } else {
            new_items.push(item.clone());
        }
    }

    if has_use_statements {
        let prelude_use: Item = syn::parse_quote! {
            use crate::prelude::*;
        };
        new_items.insert(0, prelude_use);

        let mut new_ast = ast.clone();
        new_ast.items = new_items;
        let new_content = prettyplease::unparse(&new_ast);

        if dry_run {
            println!(
                "[DRY RUN] Would modify file: {}\n---\n{}---",
                path.display(),
                new_content
            );
        } else {
            if path.exists() && !force {
                println!("  -> Skipping file modification for {} (file exists, use --force to overwrite).", path.display());
            } else {
                println!("  -> Modifying file: {}", path.display());
                println!("    -> Writing modified content to: {}", path.display());
                fs::write(path, new_content)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::io::Write;
    use std::path::PathBuf;

    fn setup_test_file(dir: &tempfile::TempDir, file_name: &str, content: &str) -> PathBuf {
        let file_path = dir.path().join(file_name);
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file_path
    }

    #[test]
    fn test_modify_file_adds_prelude_and_removes_uses() -> Result<()> {
        let dir = tempdir()?;
        let file_path = setup_test_file(&dir, "test_file.rs",
            "use std::collections::HashMap;\nuse crate::another_module;\n\nfn main() {}\n"
        );

        modify_file(&file_path, false, true)?;

        let content = fs::read_to_string(&file_path)?;
        assert!(content.contains("use crate::prelude::*;"));
        assert!(!content.contains("use std::collections::HashMap;"));
        assert!(!content.contains("use crate::another_module;"));
        assert!(content.contains("fn main() {}"));

        Ok(())
    }

    #[test]
    fn test_modify_file_dry_run() -> Result<()> {
        let dir = tempdir()?;
        let file_path = setup_test_file(&dir, "test_file.rs",
            "use std::collections::HashMap;\nfn main() {}\n"
        );
        let original_content = fs::read_to_string(&file_path)?;

        modify_file(&file_path, true, false)?;

        let content = fs::read_to_string(&file_path)?;
        assert_eq!(content, original_content); // Content should not change in dry run

        Ok(())
    }

    #[test]
    fn test_modify_file_no_use_statements() -> Result<()> {
        let dir = tempdir()?;
        let file_path = setup_test_file(&dir, "test_file.rs",
            "fn main() {}\n"
        );
        let original_content = fs::read_to_string(&file_path)?;

        modify_file(&file_path, false, false)?;

        let content = fs::read_to_string(&file_path)?;
        assert_eq!(content, original_content); // Content should not change if no use statements

        Ok(())
    }

    #[test]
    fn test_modify_file_no_force_no_overwrite() -> Result<()> {
        let dir = tempdir()?;
        let file_path = setup_test_file(&dir, "test_file.rs",
            "use std::fmt;\nfn some_func() {}\n"
        );
        let original_content = fs::read_to_string(&file_path)?;

        // Attempt to modify without force, file exists, should skip
        modify_file(&file_path, false, false)?;
        let content_after_skip = fs::read_to_string(&file_path)?;
        assert_eq!(original_content, content_after_skip);

        Ok(())
    }

    #[test]
    fn test_modify_file_force_overwrite() -> Result<()> {
        let dir = tempdir()?;
        let file_path = setup_test_file(&dir, "test_file.rs",
            "use std::fmt;\nfn some_func() {}\n"
        );

        // Modify once with force=true
        modify_file(&file_path, false, true)?;
        let first_modified_content = fs::read_to_string(&file_path)?;
        assert!(first_modified_content.contains("use crate::prelude::*;"));
        assert!(!first_modified_content.contains("use std::fmt;"));

        // Modify again with force=true (should overwrite with same logic)
        modify_file(&file_path, false, true)?;
        let second_modified_content = fs::read_to_string(&file_path)?;
        assert_eq!(first_modified_content, second_modified_content);

        Ok(())
    }
}
