use std::fmt;

#[derive(Debug)]
pub enum CleanError {
    ConfigToolNotFound,
    CommandExecutionFailed(String),
    ConfigSaveFailed(String),
    IoError(std::io::Error),
}

impl fmt::Display for CleanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CleanError::ConfigToolNotFound => {
                write!(f, "c2rust-config not found. Please install c2rust-config first.")
            }
            CleanError::CommandExecutionFailed(msg) => {
                write!(f, "Command execution failed: {}", msg)
            }
            CleanError::ConfigSaveFailed(msg) => {
                write!(f, "Failed to save configuration: {}", msg)
            }
            CleanError::IoError(err) => {
                write!(f, "IO error: {}", err)
            }
        }
    }
}

impl std::error::Error for CleanError {}

impl From<std::io::Error> for CleanError {
    fn from(err: std::io::Error) -> Self {
        CleanError::IoError(err)
    }
}

pub type Result<T> = std::result::Result<T, CleanError>;
