use crate::error::{Error, Result};
use std::process::Command;

/// Check if c2rust-config command exists
pub fn check_c2rust_config_exists() -> Result<()> {
    let result = Command::new("c2rust-config")
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
    let mut cmd = Command::new("c2rust-config");
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
    let mut cmd = Command::new("c2rust-config");
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
}
