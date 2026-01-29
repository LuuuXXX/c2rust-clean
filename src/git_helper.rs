use crate::error::{Error, Result};
use chrono::Local;
use git2::{Repository, StatusOptions};
use log::{debug, info, warn};
use std::path::{Path, PathBuf};

/// Get the project root directory from C2RUST_PROJECT_ROOT environment variable
/// or by searching for .c2rust directory
pub fn get_project_root(start_dir: &Path) -> Result<PathBuf> {
    // First, try to get from environment variable
    if let Ok(env_root) = std::env::var("C2RUST_PROJECT_ROOT") {
        let root_path = PathBuf::from(env_root);
        if root_path.exists() {
            debug!("Using C2RUST_PROJECT_ROOT: {}", root_path.display());
            return Ok(root_path);
        } else {
            warn!(
                "C2RUST_PROJECT_ROOT points to non-existent path: {}, falling back to search",
                root_path.display()
            );
        }
    }

    // Fallback to searching for .c2rust directory
    let mut current = start_dir;
    loop {
        let c2rust_dir = current.join(".c2rust");
        if c2rust_dir.exists() && c2rust_dir.is_dir() {
            debug!("Found .c2rust directory at: {}", current.display());
            return Ok(current.to_path_buf());
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => {
                debug!("No .c2rust directory found, using start directory as root");
                return Ok(start_dir.to_path_buf());
            }
        }
    }
}

/// Check if the .c2rust directory exists at the given project root
pub fn check_c2rust_dir_exists(project_root: &Path) -> bool {
    let c2rust_dir = project_root.join(".c2rust");
    let exists = c2rust_dir.exists() && c2rust_dir.is_dir();
    debug!(".c2rust directory exists: {}", exists);
    exists
}

/// Check if there are uncommitted changes in the .c2rust directory
pub fn has_uncommitted_changes(project_root: &Path) -> Result<bool> {
    let c2rust_dir = project_root.join(".c2rust");
    
    // Check if .c2rust directory exists
    if !c2rust_dir.exists() {
        debug!(".c2rust directory does not exist");
        return Ok(false);
    }

    // Try to open the git repository
    let repo = match Repository::open(&c2rust_dir) {
        Ok(repo) => repo,
        Err(e) => {
            debug!("No git repository found in .c2rust: {}", e);
            return Ok(false);
        }
    };

    // Check for uncommitted changes
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    opts.include_ignored(false);
    
    let statuses = repo.statuses(Some(&mut opts))
        .map_err(|e| Error::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get git status: {}", e)
        )))?;

    let has_changes = !statuses.is_empty();
    debug!("Uncommitted changes in .c2rust: {}", has_changes);
    
    Ok(has_changes)
}

/// Auto-commit changes in the .c2rust directory
pub fn auto_commit_c2rust_changes(project_root: &Path) -> Result<()> {
    let c2rust_dir = project_root.join(".c2rust");
    
    // Check if .c2rust directory exists
    if !check_c2rust_dir_exists(project_root) {
        debug!(".c2rust directory does not exist, skipping auto-commit");
        return Ok(());
    }

    // Try to open the git repository
    let repo = match Repository::open(&c2rust_dir) {
        Ok(repo) => repo,
        Err(e) => {
            debug!("No git repository in .c2rust, skipping auto-commit: {}", e);
            return Ok(());
        }
    };

    // Check if there are changes to commit
    if !has_uncommitted_changes(project_root)? {
        debug!("No uncommitted changes in .c2rust, skipping auto-commit");
        return Ok(());
    }

    info!("Auto-committing changes in .c2rust directory");

    // Add all changes
    let mut index = repo.index()
        .map_err(|e| Error::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get git index: {}", e)
        )))?;
    
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
        .map_err(|e| Error::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to add files to git index: {}", e)
        )))?;
    
    index.write()
        .map_err(|e| Error::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to write git index: {}", e)
        )))?;

    // Create commit message with timestamp
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let commit_message = format!("Auto-save configuration changes - {}", timestamp);

    // Get the tree
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

    // Get signature
    let sig = match repo.signature() {
        Ok(sig) => sig,
        Err(_) => {
            // Fallback to default signature
            git2::Signature::now("c2rust-clean", "c2rust-clean@auto")
                .map_err(|e| Error::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to create git signature: {}", e)
                )))?
        }
    };

    // Get HEAD commit if it exists
    let parent_commit = match repo.head() {
        Ok(head_ref) => {
            let oid = head_ref.target().ok_or_else(|| {
                Error::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "HEAD reference has no target"
                ))
            })?;
            Some(repo.find_commit(oid)
                .map_err(|e| Error::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to find HEAD commit: {}", e)
                )))?)
        }
        Err(_) => None,
    };

    // Create the commit
    let parents: Vec<&git2::Commit> = parent_commit.iter().collect();
    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &commit_message,
        &tree,
        &parents
    ).map_err(|e| Error::IoError(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Failed to create commit: {}", e)
    )))?;

    info!("Successfully committed changes to .c2rust with message: {}", commit_message);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_get_project_root_from_env() {
        let temp_dir = TempDir::new().unwrap();
        let original = std::env::var("C2RUST_PROJECT_ROOT").ok();
        
        std::env::set_var("C2RUST_PROJECT_ROOT", temp_dir.path());
        let result = get_project_root(temp_dir.path());
        
        match original {
            Some(val) => std::env::set_var("C2RUST_PROJECT_ROOT", val),
            None => std::env::remove_var("C2RUST_PROJECT_ROOT"),
        }
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_dir.path());
    }

    #[test]
    fn test_get_project_root_fallback_to_search() {
        let temp_dir = TempDir::new().unwrap();
        let original = std::env::var("C2RUST_PROJECT_ROOT").ok();
        
        std::env::remove_var("C2RUST_PROJECT_ROOT");
        
        // Create .c2rust directory
        let c2rust_dir = temp_dir.path().join(".c2rust");
        fs::create_dir(&c2rust_dir).unwrap();
        
        let result = get_project_root(temp_dir.path());
        
        match original {
            Some(val) => std::env::set_var("C2RUST_PROJECT_ROOT", val),
            None => std::env::remove_var("C2RUST_PROJECT_ROOT"),
        }
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_dir.path());
    }

    #[test]
    fn test_check_c2rust_dir_exists() {
        let temp_dir = TempDir::new().unwrap();
        
        // Should not exist initially
        assert!(!check_c2rust_dir_exists(temp_dir.path()));
        
        // Create .c2rust directory
        let c2rust_dir = temp_dir.path().join(".c2rust");
        fs::create_dir(&c2rust_dir).unwrap();
        
        // Should exist now
        assert!(check_c2rust_dir_exists(temp_dir.path()));
    }

    #[test]
    fn test_has_uncommitted_changes_no_git() {
        let temp_dir = TempDir::new().unwrap();
        let c2rust_dir = temp_dir.path().join(".c2rust");
        fs::create_dir(&c2rust_dir).unwrap();
        
        // No git repository, should return false
        let result = has_uncommitted_changes(temp_dir.path());
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_auto_commit_no_c2rust_dir() {
        let temp_dir = TempDir::new().unwrap();
        
        // Should succeed without error when .c2rust doesn't exist
        let result = auto_commit_c2rust_changes(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_auto_commit_no_git_repo() {
        let temp_dir = TempDir::new().unwrap();
        let c2rust_dir = temp_dir.path().join(".c2rust");
        fs::create_dir(&c2rust_dir).unwrap();
        
        // Should succeed without error when .c2rust has no git repo
        let result = auto_commit_c2rust_changes(temp_dir.path());
        assert!(result.is_ok());
    }
}
