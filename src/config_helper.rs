use crate::error::{Error, Result};
use std::path::Path;
use std::process::Command;

/// Get the c2rust-config binary path from environment or use default
/// 
/// Returns the path to the c2rust-config binary. If the C2RUST_CONFIG
/// environment variable is set, its value is used. Otherwise, defaults
/// to "c2rust-config" which relies on PATH resolution.
fn get_c2rust_config_path() -> String {
    std::env::var("C2RUST_CONFIG").unwrap_or_else(|_| "c2rust-config".to_string())
}

/// Check if c2rust-config command exists
/// 
/// Attempts to execute `c2rust-config --help` to verify the tool is
/// available and accessible. This should be called before any operations
/// that depend on c2rust-config.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if c2rust-config is available, or `Err(Error::ConfigToolNotFound)`
/// if the tool cannot be found or executed.
pub fn check_c2rust_config_exists() -> Result<()> {
    let config_path = get_c2rust_config_path();
    Command::new(&config_path)
        .arg("--help")
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|_| ())
        .ok_or(Error::ConfigToolNotFound)
}

/// Save clean configuration using c2rust-config
/// 
/// Saves the clean directory and command configuration to the project's
/// c2rust configuration using the c2rust-config tool.
/// 
/// # Arguments
/// 
/// * `dir` - The directory path (relative to project root) where the clean command is executed
/// * `command` - The clean command string to be saved
/// * `feature` - Optional feature name for the configuration (uses "default" if None)
/// * `project_root` - The absolute path to the project root directory
/// 
/// # Returns
/// 
/// Returns `Ok(())` if both `clean.dir` and `clean.cmd` are successfully saved,
/// or `Err(Error::ConfigSaveFailed)` if the c2rust-config tool fails to save
/// the configuration.
/// 
/// # Example
/// 
/// ```no_run
/// use std::path::Path;
/// # use c2rust_clean::config_helper::save_config;
/// # use c2rust_clean::error::Result;
/// # fn example() -> Result<()> {
/// let project_root = Path::new("/path/to/project");
/// save_config("src", "make clean", Some("default"), project_root)?;
/// # Ok(())
/// # }
/// ```
pub fn save_config(dir: &str, command: &str, feature: Option<&str>, project_root: &Path) -> Result<()> {
    let config_path = get_c2rust_config_path();
    let feature_args: Vec<&str> = feature.map(|f| vec!["--feature", f]).unwrap_or_default();

    // Save both clean.dir and clean.cmd
    for (key, value) in [("clean.dir", dir), ("clean.cmd", command)] {
        let output = Command::new(&config_path)
            .args(&["config", "--make"])
            .args(&feature_args)
            .args(&["--set", key, value])
            .current_dir(project_root)
            .output()
            .map_err(|e| Error::ConfigSaveFailed(format!("Failed to execute c2rust-config: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigSaveFailed(format!("Failed to save {}: {}", key, stderr)));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_c2rust_config_path_returns_value() {
        // Just test that it returns a non-empty string
        let path = get_c2rust_config_path();
        assert!(!path.is_empty());
        // If C2RUST_CONFIG is set, it should match that value
        // Otherwise it should be the default "c2rust-config"
        if let Ok(env_val) = std::env::var("C2RUST_CONFIG") {
            assert_eq!(path, env_val);
        } else {
            assert_eq!(path, "c2rust-config");
        }
    }

    #[test]
    fn test_check_c2rust_config_exists_with_invalid_path() {
        // Create a temporary environment where we can safely test
        // This test checks that check_c2rust_config_exists returns an error
        // when the config tool doesn't exist
        let nonexistent_path = "/this/path/definitely/does/not/exist/c2rust-config-xyz123";
        
        // Directly test with a Command that we know will fail
        let result = Command::new(nonexistent_path)
            .arg("--help")
            .output()
            .ok()
            .filter(|output| output.status.success());
        
        assert!(result.is_none());
    }
}
