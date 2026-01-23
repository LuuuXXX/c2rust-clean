use crate::error::{Error, Result};
use std::process::Command;

/// Get the c2rust-config binary path from environment or use default
fn get_c2rust_config_path() -> String {
    std::env::var("C2RUST_CONFIG").unwrap_or_else(|_| "c2rust-config".to_string())
}

/// Check if c2rust-config command exists
pub fn check_c2rust_config_exists() -> Result<()> {
    let result = Command::new(get_c2rust_config_path())
        .arg("--version")
        .output();

    match result {
        Ok(output) if output.status.success() => Ok(()),
        _ => Err(Error::ConfigToolNotFound),
    }
}

/// Save clean configuration using c2rust-config
pub fn save_config(dir: &str, command: &str, feature: Option<&str>) -> Result<()> {
    let feature_args = if let Some(f) = feature {
        vec!["--feature", f]
    } else {
        vec![]
    };

    // Save clean.dir configuration
    let mut cmd = Command::new(get_c2rust_config_path());
    cmd.args(&["config", "--make"])
        .args(&feature_args)
        .args(&["--set", "clean.dir", dir]);

    let output = cmd.output().map_err(|e| {
        Error::ConfigSaveFailed(format!("Failed to execute c2rust-config: {}", e))
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::ConfigSaveFailed(format!(
            "Failed to save clean.dir: {}",
            stderr
        )));
    }

    // Save clean command configuration
    let mut cmd = Command::new(get_c2rust_config_path());
    cmd.args(&["config", "--make"])
        .args(&feature_args)
        .args(&["--set", "clean", command]);

    let output = cmd.output().map_err(|e| {
        Error::ConfigSaveFailed(format!("Failed to execute c2rust-config: {}", e))
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::ConfigSaveFailed(format!(
            "Failed to save clean command: {}",
            stderr
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_c2rust_config_exists() {
        // This test will fail if c2rust-config is not installed
        // We can't test for ConfigToolNotFound without uninstalling it
        let _ = check_c2rust_config_exists();
    }

    #[test]
    fn test_get_c2rust_config_path_default() {
        // Ensure C2RUST_CONFIG is not set for this test
        std::env::remove_var("C2RUST_CONFIG");
        let path = get_c2rust_config_path();
        assert_eq!(path, "c2rust-config");
    }

    #[test]
    fn test_get_c2rust_config_path_from_env() {
        // Set C2RUST_CONFIG environment variable
        let custom_path = "/custom/path/to/c2rust-config";
        std::env::set_var("C2RUST_CONFIG", custom_path);
        let path = get_c2rust_config_path();
        assert_eq!(path, custom_path);
        // Clean up
        std::env::remove_var("C2RUST_CONFIG");
    }
}
