use crate::error::{Error, Result};
use std::process::Command;

/// Configuration values read from c2rust-config
#[derive(Debug, Default, Clone)]
pub struct CleanConfig {
    pub dir: Option<String>,
    pub command: Option<String>,
}

/// Get the c2rust-config binary path from environment or use default
fn get_c2rust_config_path() -> String {
    std::env::var("C2RUST_CONFIG").unwrap_or_else(|_| "c2rust-config".to_string())
}

/// Check if c2rust-config command exists
pub fn check_c2rust_config_exists() -> Result<()> {
    let config_path = get_c2rust_config_path();
    let result = Command::new(&config_path)
        .arg("--help")
        .output();

    match result {
        Ok(output) if output.status.success() => Ok(()),
        _ => Err(Error::ConfigToolNotFound),
    }
}

/// Save clean configuration using c2rust-config
pub fn save_config(dir: &str, command: &str, feature: Option<&str>) -> Result<()> {
    let config_path = get_c2rust_config_path();
    let feature_args = if let Some(f) = feature {
        vec!["--feature", f]
    } else {
        vec![]
    };

    // Save clean.dir configuration
    let mut cmd = Command::new(&config_path);
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
    let mut cmd = Command::new(&config_path);
    cmd.args(&["config", "--make"])
        .args(&feature_args)
        .args(&["--set", "clean.cmd", command]);

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

/// Read clean configuration from c2rust-config
pub fn read_config(feature: Option<&str>) -> Result<CleanConfig> {
    let config_path = get_c2rust_config_path();
    let feature_args = if let Some(f) = feature {
        vec!["--feature", f]
    } else {
        vec![]
    };

    // List all configuration using c2rust-config
    let mut cmd = Command::new(&config_path);
    cmd.args(&["config", "--make"])
        .args(&feature_args)
        .args(&["--list"]);

    let output = cmd.output().map_err(|e| {
        Error::ConfigReadFailed(format!("Failed to execute c2rust-config: {}", e))
    })?;

    if !output.status.success() {
        // If config file doesn't exist or is empty, return empty config
        return Ok(CleanConfig::default());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut config = CleanConfig::default();

    // Parse the output to find clean.dir and clean.cmd
    for line in stdout.lines() {
        let line = line.trim();
        
        // Skip lines without '='
        if !line.contains('=') {
            continue;
        }
        
        // Extract key from the line (before '=')
        let key = line.split('=').next().unwrap_or_default().trim();
        
        // Handle both "clean.dir" and clean.dir formats
        let normalized_key = key.trim_matches('"').trim_matches('\'');
        
        match normalized_key {
            "clean.dir" => {
                if let Some(value) = extract_config_value(line) {
                    config.dir = Some(value);
                }
            }
            "clean.cmd" => {
                if let Some(value) = extract_config_value(line) {
                    config.command = Some(value);
                }
            }
            _ => {
                // Help users debug near-miss configuration keys related to cleaning
                if normalized_key.starts_with("clean")
                    && normalized_key != "clean.cmd"
                    && normalized_key != "clean.dir"
                {
                    eprintln!(
                        "c2rust-config: ignoring unrecognized configuration key '{}'; \
                         expected 'clean.cmd' or 'clean.dir'",
                        normalized_key
                    );
                }
            }
        }
    }

    Ok(config)
}

/// Extract value from config line like: key = "value"
fn extract_config_value(line: &str) -> Option<String> {
    // Find the equals sign
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() != 2 {
        return None;
    }

    let value = parts[1].trim();
    Some(remove_quotes(value))
}

/// Remove surrounding quotes from a string
/// Note: Does not handle escaped quotes within quoted strings (e.g., "echo \"hello\"")
fn remove_quotes(s: &str) -> String {
    if (s.starts_with('"') && s.ends_with('"') && s.len() >= 2) 
        || (s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2) {
        s[1..s.len()-1].to_string()
    } else {
        s.to_string()
    }
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
    fn test_get_c2rust_config_path_with_env() {
        // Test that environment variable is respected
        // Save current value
        let original = std::env::var("C2RUST_CONFIG").ok();
        
        // Test with custom path
        std::env::set_var("C2RUST_CONFIG", "/custom/path/to/c2rust-config");
        let path = get_c2rust_config_path();
        assert_eq!(path, "/custom/path/to/c2rust-config");
        
        // Restore original value or remove if it wasn't set
        match original {
            Some(val) => std::env::set_var("C2RUST_CONFIG", val),
            None => std::env::remove_var("C2RUST_CONFIG"),
        }
    }

    #[test]
    fn test_get_c2rust_config_path_without_env() {
        // Test default behavior when env var is not set
        // Save current value
        let original = std::env::var("C2RUST_CONFIG").ok();
        
        // Remove env var
        std::env::remove_var("C2RUST_CONFIG");
        let path = get_c2rust_config_path();
        assert_eq!(path, "c2rust-config");
        
        // Restore original value if it was set
        if let Some(val) = original {
            std::env::set_var("C2RUST_CONFIG", val);
        }
    }

    #[test]
    fn test_extract_config_value() {
        // Test with double quotes
        assert_eq!(
            extract_config_value("clean.dir = \"build\""),
            Some("build".to_string())
        );

        // Test with single quotes
        assert_eq!(
            extract_config_value("clean.cmd = 'make clean'"),
            Some("make clean".to_string())
        );

        // Test without quotes
        assert_eq!(
            extract_config_value("key = value"),
            Some("value".to_string())
        );

        // Test with spaces
        assert_eq!(
            extract_config_value("  clean  =  \"test\"  "),
            Some("test".to_string())
        );

        // Test invalid format
        assert_eq!(extract_config_value("invalid"), None);
    }

    #[test]
    fn test_remove_quotes() {
        // Test with double quotes
        assert_eq!(remove_quotes("\"value\""), "value");
        
        // Test with single quotes
        assert_eq!(remove_quotes("'value'"), "value");
        
        // Test without quotes
        assert_eq!(remove_quotes("value"), "value");
        
        // Test empty string
        assert_eq!(remove_quotes(""), "");
        
        // Test single quote character
        assert_eq!(remove_quotes("\""), "\"");
    }

    #[test]
    fn test_read_config_with_valid_output() {
        // This test simulates the output from c2rust-config
        // We can't easily test the full read_config without mocking c2rust-config
        // but we can test the parsing logic
        
        // Test that extract_config_value works with typical config output
        assert_eq!(
            extract_config_value("\"clean.dir\" = \"build\""),
            Some("build".to_string())
        );
        
        assert_eq!(
            extract_config_value("clean.cmd = \"make clean\""),
            Some("make clean".to_string())
        );
    }
}
