use anyhow::Result;
use std::fs;
use std::path::Path;

/// Generates the `prelude.rs` file for a crate.
pub fn generate_prelude(
    src_dir: &Path,
    prelude_content: &str,
    dry_run: bool,
    force: bool,
) -> Result<()> {
    println!("  -> Entering generate_prelude for src_dir: {}", src_dir.display());
    let prelude_path = src_dir.join("prelude.rs");

    if dry_run {
        println!(
            "[DRY RUN] Would generate prelude file: {}\n---\n{}---",
            prelude_path.display(),
            prelude_content
        );
    } else {
        if prelude_path.exists() && !force {
            println!("  -> Skipping prelude file generation for {} (file exists, use --force to overwrite).", prelude_path.display());
        } else {
            println!("  -> Generating prelude file: {}", prelude_path.display());
            println!("    -> Writing prelude content to: {}", prelude_path.display());
            fs::write(&prelude_path, prelude_content)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generate_prelude_creates_file() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;
        let prelude_content = "// Test prelude content";

        generate_prelude(&src_dir, prelude_content, false, false)?;

        let prelude_path = src_dir.join("prelude.rs");
        assert!(prelude_path.exists());
        assert_eq!(fs::read_to_string(&prelude_path)?, prelude_content);

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
        assert!(!prelude_path.exists()); // File should not be created in dry run

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
        assert_eq!(fs::read_to_string(&prelude_path)?, new_prelude_content);

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
        assert_eq!(fs::read_to_string(&prelude_path)?, original_content); // Content should not change

        Ok(())
    }
}
