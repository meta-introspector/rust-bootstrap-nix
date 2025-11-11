use crate::prelude::*;
use git_utils;
use anyhow::Result;

use std::path::{Path, PathBuf};
use std::process::Command;

pub fn run_git_command(
    current_dir: &Path,
    args: &[&str],
    error_message: &str,
    dry_run: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running git command in CWD: {:?}", current_dir);
    let command_str = format!("git {}", args.join(" "));
    if dry_run {
        println!("Dry run: Would execute: {}", command_str);
        return Ok(());
    }
    println!("Executing: {}", command_str);

    let output = Command::new("git")
        .current_dir(current_dir)
        .args(args)
        .output()?;

    if !output.status.success() {
        eprintln!("Git command failed: {}", error_message);
        eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        return Err(error_message.into());
    }
    Ok(())
}

pub fn create_and_push_branch(
    repo_root: &PathBuf,
    branch_name: &str,
    base_branch_name: &str,
    output_dir: &PathBuf,
    dry_run: bool,
) -> Result<()> {
    println!("Performing Git operations...");

    // Explicitly checkout the base branch to ensure a stable HEAD
    run_git_command(repo_root, &["checkout", base_branch_name], "Failed to checkout base branch", dry_run)?;

    // Check if branch already exists
    let branch_exists_output = Command::new("git")
        .current_dir(repo_root)
        .args(&["rev-parse", "--verify", branch_name])
        .output()?;

    if branch_exists_output.status.success() {
        println!("Branch '{}' already exists. Checking it out.", branch_name);
        run_git_command(repo_root, &["checkout", branch_name], "Failed to checkout existing branch", dry_run)?;
    } else {
        println!("Branch '{}' does not exist. Creating and checking it out.", branch_name);
        run_git_command(repo_root, &["checkout", "-b", branch_name], "Failed to create and checkout new branch", dry_run)?;
    }

    // Add generated files
    if dry_run {
        println!("Dry run: Would add all files in '{}'", repo_root.display());
    } else {
        git_utils::add_all(repo_root)?;
    }

    // Commit changes
    let commit_message = format!("feat: Generated seed flake {}", branch_name);
    if dry_run {
        println!("Dry run: Would commit with message: '{}'", commit_message);
    } else {
        // Assuming a default author for now, this should ideally be configurable
        git_utils::commit_files(repo_root, &commit_message, "Rust Bootstrap", "rust-bootstrap@example.com")?;
    }

    // Push branch
    run_git_command(repo_root, &["push", "origin", branch_name], "Failed to push branch", dry_run)?;
    println!("Successfully pushed branch: {}", branch_name);

    Ok(())
}
