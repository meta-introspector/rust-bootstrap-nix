use git2::{Repository, Signature, IndexAddOption};
use anyhow::{Result, Context};
use log::info;
use std::path::Path;

pub fn commit_files(repo_path: &Path, message: &str, author_name: &str, author_email: &str) -> Result<()> {
    let repo = Repository::open(repo_path).context("Failed to open repository")?;
    let mut index = repo.index().context("Failed to get index")?;
    let oid = index.write_tree().context("Failed to write tree")?;
    let signature = Signature::now(author_name, author_email).context("Failed to create signature")?;
    let parent_commit = find_last_commit(&repo).context("Failed to find last commit")?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &repo.find_tree(oid).context("Failed to find tree")?,
        &[&parent_commit],
    ).context("Failed to commit")?;
    info!("Committed changes to '{}'", repo_path.display());
    Ok(())
}

fn find_last_commit(repo: &Repository) -> Result<git2::Commit<'_>> {
    let obj = repo.head().context("Failed to get HEAD")?.resolve().context("Failed to resolve HEAD")?.peel(git2::ObjectType::Commit).context("Failed to peel HEAD to commit")?;
    obj.into_commit().map_err(|_| anyhow::anyhow!("Could not convert object to commit"))
}

pub fn init_repo(repo_path: &Path) -> Result<()> {
    Repository::init(repo_path).context("Failed to initialize repository")?;
    info!("Initialized Git repository in '{}'", repo_path.display());
    Ok(())
}

pub fn add_all(repo_path: &Path) -> Result<()> {
    let repo = Repository::open(repo_path).context("Failed to open repository")?;
    let mut index = repo.index().context("Failed to get index")?;
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None).context("Failed to add all files")?;
    index.write().context("Failed to write index")?;
    info!("Added all files in '{}' to the staging area", repo_path.display());
    Ok(())
}
