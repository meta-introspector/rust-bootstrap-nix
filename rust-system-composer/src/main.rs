pub mod prelude;
use crate::prelude::*;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the workspace root for prelude-generator.
    #[arg(long)]
    workspace_root: PathBuf,
    /// The input directory for rust-decl-splitter.
    #[arg(long)]
    input_dir: PathBuf,
    /// The output directory for rust-decl-splitter.
    #[arg(long)]
    output_dir: PathBuf,
    /// Run in dry-run mode, printing changes without modifying files.
    #[arg(long)]
    dry_run: bool,
}
fn main() -> Result<()> {
    let args = Args::parse();
    println!("Running prelude-generator...");
    let mut prelude_cmd = Command::new("cargo");
    prelude_cmd
        .arg("run")
        .arg("-p")
        .arg("prelude-generator")
        .arg("--")
        .arg(&args.workspace_root)
        .arg("--exclude-crates")
        .arg("bootstrap-test-utils,serde_core,bootstrap");
    if args.dry_run {
        prelude_cmd.arg("--dry-run");
    }
    let prelude_output = prelude_cmd
        .output()
        .context("Failed to execute prelude-generator")?;
    if !prelude_output.status.success() {
        anyhow::bail!(
            "prelude-generator failed: {}\n{}", String::from_utf8_lossy(& prelude_output
            .stdout), String::from_utf8_lossy(& prelude_output.stderr)
        );
    }
    println!(
        "prelude-generator output:\n{}", String::from_utf8_lossy(& prelude_output.stdout)
    );
    eprintln!(
        "prelude-generator errors:\n{}", String::from_utf8_lossy(& prelude_output.stderr)
    );
    println!("\nRunning rust-decl-splitter...");
    let mut splitter_cmd = Command::new("cargo");
    splitter_cmd
        .arg("run")
        .arg("-p")
        .arg("rust-decl-splitter")
        .arg("--")
        .arg(&args.input_dir)
        .arg(&args.output_dir);
    let splitter_output = splitter_cmd
        .output()
        .context("Failed to execute rust-decl-splitter")?;
    if !splitter_output.status.success() {
        anyhow::bail!(
            "rust-decl-splitter failed: {}\n{}", String::from_utf8_lossy(&
            splitter_output.stdout), String::from_utf8_lossy(& splitter_output.stderr)
        );
    }
    println!(
        "rust-decl-splitter output:\n{}", String::from_utf8_lossy(& splitter_output
        .stdout)
    );
    eprintln!(
        "rust-decl-splitter errors:\n{}", String::from_utf8_lossy(& splitter_output
        .stderr)
    );
    println!("\nSystem composition complete.");
    Ok(())
}
