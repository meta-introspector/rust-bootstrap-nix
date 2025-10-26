


use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    rust_version: String,
    #[clap(long)]
    arch: String,
    #[clap(long)]
    phase: String,
    #[clap(long)]
    step: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Parsed arguments:");
    println!("  Rust Version: {}", args.rust_version);
    println!("  Architecture: {}", args.arch);
    println!("  Phase: {}", args.phase);
    println!("  Step: {}", args.step);

    Ok(())
}
