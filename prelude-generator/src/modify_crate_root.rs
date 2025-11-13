use anyhow::Result;
use std::fs;
use std::path::Path;
use syn::Item;
use prettyplease;

/// Modifies the crate root (`lib.rs` or `main.rs`) to ensure it contains `pub mod prelude;`.
pub fn modify_crate_root(src_dir: &Path, dry_run: bool, force: bool) -> Result<()> {
    println!("  -> Entering modify_crate_root for src_dir: {}", src_dir.display());
    let lib_rs = src_dir.join("lib.rs");
    let main_rs = src_dir.join("main.rs");

    let crate_root_path = if lib_rs.exists() {
        lib_rs
    } else if main_rs.exists() {
        main_rs
    } else {
        return Ok(());
    };

    let content = fs::read_to_string(&crate_root_path)?;
    let ast = syn::parse_file(&content)?;
    let mut has_prelude_mod = false;

    for item in &ast.items {
        if let Item::Mod(mod_item) = item {
            if mod_item.ident == "prelude" {
                has_prelude_mod = true;
                break;
            }
        }
    }

    if !has_prelude_mod {
        let mut new_ast = ast.clone();
        let prelude_mod: Item = syn::parse_quote! {
            pub mod prelude;
        };
        new_ast.items.insert(0, prelude_mod);
        let new_content = prettyplease::unparse(&new_ast);

        if dry_run {
            println!(
                "[DRY RUN] Would add 'pub mod prelude;' to: {}",
                crate_root_path.display()
            );
        } else {
            if crate_root_path.exists() && !force {
                println!("  -> Skipping crate root modification for {} (file exists, use --force to overwrite).", crate_root_path.display());
            } else {
                println!("  -> Adding 'pub mod prelude;' to: {}", crate_root_path.display());
                println!("    -> Writing modified content to: {}", crate_root_path.display());
                fs::write(&crate_root_path, new_content)?;
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
        let lib_rs_path = setup_test_file(&dir, "src/lib.rs", "pub mod prelude;\nfn main() {}\n");
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

        // First modification (adds prelude)
        modify_crate_root(&src_dir, false, true)?;
        let first_modified_content = fs::read_to_string(&lib_rs_path)?;
        assert!(first_modified_content.contains("pub mod prelude;"));

        // Attempt to modify again without force, should skip
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

        // Modify once with force=true
        modify_crate_root(&src_dir, false, true)?;
        let first_modified_content = fs::read_to_string(&lib_rs_path)?;
        assert!(first_modified_content.contains("pub mod prelude;"));

        // Modify again with force=true (should overwrite with same logic)
        modify_crate_root(&src_dir, false, true)?;
        let second_modified_content = fs::read_to_string(&lib_rs_path)?;
        assert_eq!(first_modified_content, second_modified_content);

        Ok(())
    }
}
