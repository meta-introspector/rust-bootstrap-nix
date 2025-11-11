use git2::{Repository, Signature, IndexAddOption};
use anyhow::{Result, Context};
use log::info;
use std::path::Path;

pub fn commit_files(repo_path: &Path, message: &str, author_name: &str, author_email: &str) -> Result<()> {
    let repo = Repository::open(repo_path).context("Failed to open repository")?;
    let mut index = repo.index().context("Failed to get index")?;
    let oid = index.write_tree().context("Failed to write tree")?;
    let signature = Signature::now(author_name, author_email).context("Failed to create signature")?;

    let tree = repo.find_tree(oid).context("Failed to find tree")?;

    let parent_commit_opt = find_last_commit(&repo).ok(); // Try to find a parent commit

    let commit_oid = match parent_commit_opt {
        Some(parent_commit) => {
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                message,
                &tree,
                &[&parent_commit],
            ).context("Failed to commit with parent")?
        },
        None => {
            // No parent commit, this is the first commit
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                message,
                &tree,
                &[], // No parents for the initial commit
            ).context("Failed to make initial commit")?
        }
    };

    info!("Committed changes to '{}' with OID {}", repo_path.display(), commit_oid);
    Ok(())
}

fn find_last_commit(repo: &Repository) -> Result<git2::Commit<'_>> {
    let obj = repo.head()?.resolve()?.peel(git2::ObjectType::Commit)?;
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
