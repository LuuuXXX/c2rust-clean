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
        // Save the current C2RUST_CONFIG value to restore after the test
        let original = std::env::var("C2RUST_CONFIG").ok();

        // Point C2RUST_CONFIG to a path that definitely does not exist
        let nonexistent_path = "/this/path/definitely/does/not/exist/c2rust-config-xyz123";
        std::env::set_var("C2RUST_CONFIG", nonexistent_path);

        // Now check that the helper reports the tool as not found
        let result = check_c2rust_config_exists();
        match result {
            Err(Error::ConfigToolNotFound) => {}
            other => panic!(
                "expected Err(Error::ConfigToolNotFound), got {:?}",
                other
            ),
        }

        // Restore the original C2RUST_CONFIG value
        match original {
            Some(val) => std::env::set_var("C2RUST_CONFIG", val),
            None => std::env::remove_var("C2RUST_CONFIG"),
        }
    }

    #[test]
    fn test_save_config_with_mock_success() {
        use std::env;
        use std::fs;
        use std::io::Write;
        use tempfile::TempDir;

        // Save the current C2RUST_CONFIG value
        let original = env::var("C2RUST_CONFIG").ok();

        // Create a temp directory and mock c2rust-config script
        let temp_dir = TempDir::new().unwrap();
        let mock_script_path = temp_dir.path().join("mock-c2rust-config");

        #[cfg(unix)]
        {
            let mut script = fs::File::create(&mock_script_path).unwrap();
            writeln!(script, "#!/bin/bash").unwrap();
            writeln!(script, "exit 0").unwrap();
            
            // Make script executable
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&mock_script_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&mock_script_path, perms).unwrap();
        }

        #[cfg(windows)]
        {
            let mut script = fs::File::create(&mock_script_path.with_extension("bat")).unwrap();
            writeln!(script, "@echo off").unwrap();
            writeln!(script, "exit /b 0").unwrap();
        }

        // Point C2RUST_CONFIG to our mock script
        #[cfg(unix)]
        env::set_var("C2RUST_CONFIG", &mock_script_path);
        #[cfg(windows)]
        env::set_var("C2RUST_CONFIG", mock_script_path.with_extension("bat"));

        // Test save_config
        let project_root = temp_dir.path();
        let result = save_config("src", "make clean", Some("default"), project_root);

        // Should succeed
        assert!(result.is_ok(), "Expected save_config to succeed, got: {:?}", result);

        // Restore the original C2RUST_CONFIG value
        match original {
            Some(val) => env::set_var("C2RUST_CONFIG", val),
            None => env::remove_var("C2RUST_CONFIG"),
        }
    }

    #[test]
    fn test_save_config_failure() {
        use std::env;
        use std::fs;
        use std::io::Write;
        use tempfile::TempDir;

        // Save the current C2RUST_CONFIG value
        let original = env::var("C2RUST_CONFIG").ok();

        // Create a temp directory and mock c2rust-config script that fails
        let temp_dir = TempDir::new().unwrap();
        let mock_script_path = temp_dir.path().join("mock-c2rust-config-fail");

        #[cfg(unix)]
        {
            let mut script = fs::File::create(&mock_script_path).unwrap();
            writeln!(script, "#!/bin/bash").unwrap();
            writeln!(script, "echo 'Error: failed to save config' >&2").unwrap();
            writeln!(script, "exit 1").unwrap();
            
            // Make script executable
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&mock_script_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&mock_script_path, perms).unwrap();
        }

        #[cfg(windows)]
        {
            let mut script = fs::File::create(&mock_script_path.with_extension("bat")).unwrap();
            writeln!(script, "@echo off").unwrap();
            writeln!(script, "echo Error: failed to save config 1>&2").unwrap();
            writeln!(script, "exit /b 1").unwrap();
        }

        // Point C2RUST_CONFIG to our failing mock script
        #[cfg(unix)]
        env::set_var("C2RUST_CONFIG", &mock_script_path);
        #[cfg(windows)]
        env::set_var("C2RUST_CONFIG", mock_script_path.with_extension("bat"));

        // Test save_config
        let project_root = temp_dir.path();
        let result = save_config("src", "make clean", Some("default"), project_root);

        // Should fail with ConfigSaveFailed
        match result {
            Err(Error::ConfigSaveFailed(msg)) => {
                assert!(msg.contains("Failed to save clean.dir"), 
                       "Expected error message about clean.dir, got: {}", msg);
            }
            other => panic!("Expected Err(ConfigSaveFailed), got: {:?}", other),
        }

        // Restore the original C2RUST_CONFIG value
        match original {
            Some(val) => env::set_var("C2RUST_CONFIG", val),
            None => env::remove_var("C2RUST_CONFIG"),
        }
    }
}
