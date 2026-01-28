use crate::error::{Error, Result};
use std::path::Path;
use std::process::Command;

/// Get the c2rust-config binary path from environment or use default
fn get_c2rust_config_path() -> String {
    std::env::var("C2RUST_CONFIG").unwrap_or_else(|_| "c2rust-config".to_string())
}

/// Check if c2rust-config command exists
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
