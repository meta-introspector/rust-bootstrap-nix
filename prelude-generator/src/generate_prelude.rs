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
