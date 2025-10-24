use anyhow::{Result};
use git2::{Repository, Signature, Oid};
use log::info;

pub fn create_orphan_branch(repo_path: &str, branch_name: &str) -> Result<()> {
    let repo = Repository::open(repo_path)?;
    info!("Creating orphan branch '{}' in repo at '{}'", branch_name, repo_path);

    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;

    let signature = Signature::now("bootstrap-config-generator", "bootstrap-config-generator@example.com")?;

    // Create an empty tree
    let tree_id = Oid::from_str("4b825dc642cb6eb9a060e54bf8d69288fbee4904")?;
    let tree = repo.find_tree(tree_id)?;

    // Create the commit
    let commit_id = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit for orphan branch",
        &tree,
        &[&head_commit],
    )?;

    // Create the branch
    repo.branch(branch_name, &repo.find_commit(commit_id)?, true)?;

    Ok(())
}

pub fn commit_files(repo_path: &str, files: &[&str], message: &str) -> Result<()> {
    let repo = Repository::open(repo_path)?;
    let mut index = repo.index()?;

    for file in files {
        index.add_path(std::path::Path::new(file))?;
    }

    index.write()?;

    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;

    let signature = Signature::now("bootstrap-config-generator", "bootstrap-config-generator@example.com")?;
    let parent_commit = repo.head()?.peel_to_commit()?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent_commit],
    )?;

    Ok(())
}
