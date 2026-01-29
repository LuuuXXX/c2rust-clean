use crate::error::{Error, Result};
use git2::{Repository, Status, StatusOptions};
use std::path::Path;

/// Check if there are any changes in the .c2rust directory and auto-commit them
///
/// This function detects any changes (new, modified, or deleted files) in the
/// .c2rust directory and automatically commits them to the .c2rust/.git repository.
///
/// # Arguments
///
/// * `project_root` - The path to the project root directory
///
/// # Returns
///
/// Returns `Ok(())` if the operation succeeds (whether or not there were changes to commit),
/// or an error if Git operations fail.
pub fn auto_commit_c2rust_changes(project_root: &Path) -> Result<()> {
    let c2rust_dir = project_root.join(".c2rust");
    
    // Check if .c2rust directory exists
    if !c2rust_dir.exists() || !c2rust_dir.is_dir() {
        // No .c2rust directory, nothing to commit
        return Ok(());
    }

    // Open the repository at .c2rust directory
    // If .c2rust exists but is not a git repository, skip auto-commit
    let repo = match Repository::open(&c2rust_dir) {
        Ok(repo) => repo,
        Err(_) => {
            // .c2rust exists but is not a git repository, skip auto-commit
            return Ok(());
        }
    };

    // Check if there are any changes
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    opts.recurse_untracked_dirs(true);
    
    let statuses = repo.statuses(Some(&mut opts)).map_err(|e| {
        Error::GitOperationFailed(format!("Failed to get repository status: {}", e))
    })?;

    // Check if there are any changes
    let has_changes = statuses.iter().any(|entry| {
        let status = entry.status();
        status.intersects(
            Status::WT_NEW
                | Status::WT_MODIFIED
                | Status::WT_DELETED
                | Status::WT_RENAMED
                | Status::INDEX_NEW
                | Status::INDEX_MODIFIED
                | Status::INDEX_DELETED
                | Status::INDEX_RENAMED,
        )
    });

    if !has_changes {
        // No changes to commit
        return Ok(());
    }

    // Stage all changes
    let mut index = repo.index().map_err(|e| {
        Error::GitOperationFailed(format!("Failed to get repository index: {}", e))
    })?;

    // Add all changes to the index
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None).map_err(|e| {
        Error::GitOperationFailed(format!("Failed to add changes to index: {}", e))
    })?;

    index.write().map_err(|e| {
        Error::GitOperationFailed(format!("Failed to write index: {}", e))
    })?;

    // Create a commit
    let tree_id = index.write_tree().map_err(|e| {
        Error::GitOperationFailed(format!("Failed to write tree: {}", e))
    })?;

    let tree = repo.find_tree(tree_id).map_err(|e| {
        Error::GitOperationFailed(format!("Failed to find tree: {}", e))
    })?;

    let signature = repo.signature().map_err(|e| {
        Error::GitOperationFailed(format!("Failed to create signature: {}", e))
    })?;

    // Get the HEAD commit if it exists
    let parent_commit = repo.head()
        .ok()
        .and_then(|head| head.peel_to_commit().ok());

    let parents: Vec<&git2::Commit> = parent_commit.as_ref().map(|c| vec![c]).unwrap_or_default();

    let commit_message = "Auto-commit: Save c2rust clean configuration changes";

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        commit_message,
        &tree,
        &parents,
    )
    .map_err(|e| {
        Error::GitOperationFailed(format!("Failed to create commit: {}", e))
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_auto_commit_no_c2rust_dir() {
        let temp_dir = TempDir::new().unwrap();
        // No .c2rust directory exists
        let result = auto_commit_c2rust_changes(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_auto_commit_with_git_repo() {
        let temp_dir = TempDir::new().unwrap();
        let c2rust_dir = temp_dir.path().join(".c2rust");
        fs::create_dir(&c2rust_dir).unwrap();

        // Initialize a git repository in .c2rust
        let repo = Repository::init(&c2rust_dir).unwrap();

        // Configure git user for the test repository
        let mut config = repo.config().unwrap();
        config.set_str("user.name", "Test User").unwrap();
        config.set_str("user.email", "test@example.com").unwrap();

        // Create a test file
        let test_file = c2rust_dir.join("test.txt");
        fs::write(&test_file, "test content").unwrap();

        // Auto-commit should work
        let result = auto_commit_c2rust_changes(temp_dir.path());
        assert!(result.is_ok());

        // Verify that the commit was created
        let repo = Repository::open(&c2rust_dir).unwrap();
        let head = repo.head().unwrap();
        let commit = head.peel_to_commit().unwrap();
        assert!(commit.message().unwrap().contains("Auto-commit"));
    }

    #[test]
    fn test_auto_commit_no_changes() {
        let temp_dir = TempDir::new().unwrap();
        let c2rust_dir = temp_dir.path().join(".c2rust");
        fs::create_dir(&c2rust_dir).unwrap();

        // Initialize a git repository in .c2rust
        let repo = Repository::init(&c2rust_dir).unwrap();

        // Configure git user for the test repository
        let mut config = repo.config().unwrap();
        config.set_str("user.name", "Test User").unwrap();
        config.set_str("user.email", "test@example.com").unwrap();

        // Create and commit a test file
        let test_file = c2rust_dir.join("test.txt");
        fs::write(&test_file, "test content").unwrap();

        let mut index = repo.index().unwrap();
        index.add_path(Path::new("test.txt")).unwrap();
        index.write().unwrap();

        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let signature = repo.signature().unwrap();

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[],
        )
        .unwrap();

        // Now call auto_commit with no new changes
        let result = auto_commit_c2rust_changes(temp_dir.path());
        assert!(result.is_ok());

        // Verify that no new commit was created (still just the initial commit)
        let head = repo.head().unwrap();
        let commit = head.peel_to_commit().unwrap();
        assert_eq!(commit.message().unwrap(), "Initial commit");
    }
}
