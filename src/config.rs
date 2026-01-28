use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const C2RUST_DIR: &str = ".c2rust";
const CONFIG_FILE: &str = "config.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// The build directory relative to the .c2rust folder location
    pub build_dir: String,
}

impl Config {
    /// Find the .c2rust directory by searching up from the current directory
    pub fn find_c2rust_root() -> Result<PathBuf> {
        let current_dir = std::env::current_dir()?;
        let mut path = current_dir.as_path();

        loop {
            let c2rust_path = path.join(C2RUST_DIR);
            if c2rust_path.exists() && c2rust_path.is_dir() {
                return Ok(path.to_path_buf());
            }

            match path.parent() {
                Some(parent) => path = parent,
                None => {
                    return Err(Error::IoError(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!(
                            "Could not find '{}' directory in current path or any parent directory",
                            C2RUST_DIR
                        ),
                    )))
                }
            }
        }
    }

    /// Calculate the current directory relative to the .c2rust root
    pub fn calculate_relative_dir(c2rust_root: &Path) -> Result<String> {
        let current_dir = std::env::current_dir()?;
        
        match current_dir.strip_prefix(c2rust_root) {
            Ok(relative) => {
                let rel_str = relative.to_str().ok_or_else(|| {
                    Error::IoError(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Path contains invalid UTF-8",
                    ))
                })?;
                
                // Return "." if we're at the root
                if rel_str.is_empty() {
                    Ok(".".to_string())
                } else {
                    Ok(rel_str.to_string())
                }
            }
            Err(_) => Err(Error::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "Current directory is not under the .c2rust root: {}",
                    c2rust_root.display()
                ),
            ))),
        }
    }

    /// Save the build directory configuration
    pub fn save(c2rust_root: &Path, build_dir: &str) -> Result<()> {
        let c2rust_path = c2rust_root.join(C2RUST_DIR);
        
        // Create .c2rust directory if it doesn't exist
        if !c2rust_path.exists() {
            fs::create_dir_all(&c2rust_path)?;
        }

        let config = Config {
            build_dir: build_dir.to_string(),
        };

        let config_path = c2rust_path.join(CONFIG_FILE);
        let json = serde_json::to_string_pretty(&config).map_err(|e| {
            Error::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to serialize config: {}", e),
            ))
        })?;

        fs::write(config_path, json)?;
        Ok(())
    }

    /// Load the build directory configuration
    pub fn load(c2rust_root: &Path) -> Result<Config> {
        let config_path = c2rust_root.join(C2RUST_DIR).join(CONFIG_FILE);

        if !config_path.exists() {
            return Err(Error::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Configuration file not found: {}. Please run the command from a build directory first to save the configuration.",
                    config_path.display()
                ),
            )));
        }

        let json = fs::read_to_string(&config_path)?;
        let config: Config = serde_json::from_str(&json).map_err(|e| {
            Error::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to parse config: {}", e),
            ))
        })?;

        Ok(config)
    }

    /// Get the absolute build directory path
    pub fn get_build_dir_path(c2rust_root: &Path, config: &Config) -> PathBuf {
        c2rust_root.join(&config.build_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_calculate_relative_dir() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let root = temp_dir.path();
        let sub_dir = root.join("sub").join("dir");
        fs::create_dir_all(&sub_dir).unwrap();

        // Save and restore current dir
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&sub_dir).unwrap();

        let relative = Config::calculate_relative_dir(root).unwrap();
        assert_eq!(relative, "sub/dir");

        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let root = temp_dir.path();

        Config::save(root, "build").unwrap();
        let config = Config::load(root).unwrap();
        assert_eq!(config.build_dir, "build");
    }
}
