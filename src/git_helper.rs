use crate::error::{Error, Result};
use git2::{Repository, Signature, StatusOptions};
use std::path::PathBuf;

/// Get the project root directory from C2RUST_PROJECT_ROOT environment variable
/// 
/// Returns the path to the project root directory. The C2RUST_PROJECT_ROOT
/// environment variable must be set and point to a valid directory.
/// 
/// # Returns
/// 
/// Returns `Ok(PathBuf)` if the environment variable is set and points to a valid directory,
/// or `Err(Error::IoError)` if the environment variable is not set or the path is invalid.
fn get_project_root() -> Result<PathBuf> {
    let root = std::env::var("C2RUST_PROJECT_ROOT")
        .map_err(|_| Error::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "C2RUST_PROJECT_ROOT environment variable not set"
        )))?;
    
    let root_path = PathBuf::from(root);
    if !root_path.exists() || !root_path.is_dir() {
        return Err(Error::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("C2RUST_PROJECT_ROOT path does not exist or is not a directory: {}", root_path.display())
        )));
    }
    
    Ok(root_path)
}

/// Get the path to the .c2rust directory
fn get_c2rust_dir() -> Result<PathBuf> {
    let project_root = get_project_root()?;
    Ok(project_root.join(".c2rust"))
}

/// Initialize git repository in .c2rust directory if it doesn't exist
/// 
/// Creates a git repository in `<C2RUST_PROJECT_ROOT>/.c2rust/.git` if it doesn't already exist.
/// 
/// # Returns
/// 
/// Returns `Ok(Repository)` with the initialized or existing repository,
/// or `Err(Error::IoError)` if initialization fails.
fn ensure_git_repo() -> Result<Repository> {
    let c2rust_dir = get_c2rust_dir()?;
    
    // Create .c2rust directory if it doesn't exist
    if !c2rust_dir.exists() {
        std::fs::create_dir_all(&c2rust_dir)?;
    }
    
    // Try to open existing repository first
    match Repository::open(&c2rust_dir) {
        Ok(repo) => Ok(repo),
        Err(_) => {
            // Repository doesn't exist, initialize it
            Repository::init(&c2rust_dir)
                .map_err(|e| Error::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to initialize git repository: {}", e)
                )))
        }
    }
}

/// Check if there are any modifications in the .c2rust directory
/// 
/// Checks the git status of the .c2rust directory to detect any changes.
/// 
/// # Returns
/// 
/// Returns `Ok(true)` if there are modifications, `Ok(false)` if there are no modifications,
/// or `Err(Error::IoError)` if the check fails.
fn has_modifications(repo: &Repository) -> Result<bool> {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    opts.include_ignored(false);
    
    let statuses = repo.statuses(Some(&mut opts))
        .map_err(|e| Error::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get git status: {}", e)
        )))?;
    
    // Check if there are any changes (new, modified, or deleted files)
    Ok(!statuses.is_empty())
}

/// Commit all changes in the .c2rust directory
/// 
/// Stages and commits all changes in the .c2rust directory with a timestamp-based message.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the commit succeeds, or `Err(Error::IoError)` if the commit fails.
fn commit_changes(repo: &Repository) -> Result<()> {
    let mut index = repo.index()
        .map_err(|e| Error::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get repository index: {}", e)
        )))?;
    
    // Add all files to the index
    index.add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None)
        .map_err(|e| Error::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to add files to index: {}", e)
        )))?;
    
    index.write()
        .map_err(|e| Error::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to write index: {}", e)
        )))?;
    
    let tree_id = index.write_tree()
        .map_err(|e| Error::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to write tree: {}", e)
        )))?;
    
    let tree = repo.find_tree(tree_id)
        .map_err(|e| Error::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to find tree: {}", e)
        )))?;
    
    let signature = Signature::now("c2rust-clean", "c2rust-clean@auto")
        .map_err(|e| Error::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to create signature: {}", e)
        )))?;
    
    let message = format!("Auto-commit changes at {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    
    // Check if there's a parent commit
    let parent_commit = repo.head()
        .ok()
        .and_then(|head| head.target())
        .and_then(|oid| repo.find_commit(oid).ok());
    
    match parent_commit {
        Some(parent) => {
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &message,
                &tree,
                &[&parent]
            )
        }
        None => {
            // First commit
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &message,
                &tree,
                &[]
            )
        }
    }.map_err(|e| Error::IoError(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Failed to commit: {}", e)
    )))?;
    
    Ok(())
}

/// Check and commit changes in the .c2rust directory
/// 
/// This is the main entry point for the git_helper module. It:
/// 1. Ensures a git repository exists in `<C2RUST_PROJECT_ROOT>/.c2rust/.git`
/// 2. Checks for modifications in the .c2rust directory
/// 3. If modifications exist, commits them with an auto-generated message
/// 
/// # Returns
/// 
/// Returns `Ok(())` if successful (whether or not changes were committed),
/// or `Err(Error)` if any operation fails.
pub fn check_and_commit() -> Result<()> {
    let repo = ensure_git_repo()?;
    
    if has_modifications(&repo)? {
        commit_changes(&repo)?;
        eprintln!("✓ Changes in .c2rust directory committed to git");
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_get_project_root_not_set() {
        // Save current value
        let original = std::env::var("C2RUST_PROJECT_ROOT").ok();
        
        // Remove the environment variable
        std::env::remove_var("C2RUST_PROJECT_ROOT");
        
        let result = get_project_root();
        assert!(result.is_err());
        
        // Restore
        if let Some(val) = original {
            std::env::set_var("C2RUST_PROJECT_ROOT", val);
        }
    }

    #[test]
    fn test_get_project_root_valid() {
        let temp_dir = TempDir::new().unwrap();
        
        // Save current value
        let original = std::env::var("C2RUST_PROJECT_ROOT").ok();
        
        std::env::set_var("C2RUST_PROJECT_ROOT", temp_dir.path());
        
        let result = get_project_root();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_dir.path());
        
        // Restore
        match original {
            Some(val) => std::env::set_var("C2RUST_PROJECT_ROOT", val),
            None => std::env::remove_var("C2RUST_PROJECT_ROOT"),
        }
    }

    #[test]
    fn test_ensure_git_repo_creates_repo() {
        let temp_dir = TempDir::new().unwrap();
        
        // Save current value
        let original = std::env::var("C2RUST_PROJECT_ROOT").ok();
        
        std::env::set_var("C2RUST_PROJECT_ROOT", temp_dir.path());
        
        let result = ensure_git_repo();
        assert!(result.is_ok());
        
        // Check that .c2rust/.git exists
        let git_dir = temp_dir.path().join(".c2rust").join(".git");
        assert!(git_dir.exists());
        
        // Restore
        match original {
            Some(val) => std::env::set_var("C2RUST_PROJECT_ROOT", val),
            None => std::env::remove_var("C2RUST_PROJECT_ROOT"),
        }
    }

    #[test]
    fn test_check_and_commit_with_changes() {
        let temp_dir = TempDir::new().unwrap();
        
        // Save current value
        let original = std::env::var("C2RUST_PROJECT_ROOT").ok();
        
        std::env::set_var("C2RUST_PROJECT_ROOT", temp_dir.path());
        
        // Create .c2rust directory and add a file
        let c2rust_dir = temp_dir.path().join(".c2rust");
        fs::create_dir_all(&c2rust_dir).unwrap();
        fs::write(c2rust_dir.join("test.txt"), "test content").unwrap();
        
        // This should initialize repo and commit the file
        let result = check_and_commit();
        assert!(result.is_ok());
        
        // Verify that the file was committed
        let repo = Repository::open(&c2rust_dir).unwrap();
        let head = repo.head().unwrap();
        assert!(head.is_branch());
        
        // Restore
        match original {
            Some(val) => std::env::set_var("C2RUST_PROJECT_ROOT", val),
            None => std::env::remove_var("C2RUST_PROJECT_ROOT"),
        }
    }

    #[test]
    fn test_check_and_commit_no_changes() {
        let temp_dir = TempDir::new().unwrap();
        
        // Save current value
        let original = std::env::var("C2RUST_PROJECT_ROOT").ok();
        
        std::env::set_var("C2RUST_PROJECT_ROOT", temp_dir.path());
        
        // Initialize repo without any files
        let result = ensure_git_repo();
        assert!(result.is_ok());
        
        // This should not fail even with no changes
        let result = check_and_commit();
        assert!(result.is_ok());
        
        // Restore
        match original {
            Some(val) => std::env::set_var("C2RUST_PROJECT_ROOT", val),
            None => std::env::remove_var("C2RUST_PROJECT_ROOT"),
        }
    }
}
