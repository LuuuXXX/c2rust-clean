use crate::error::{Error, Result};
use std::process::Command;

/// Execute a command in the specified directory
pub fn execute_command(dir: &str, command: &[String]) -> Result<()> {
    if command.is_empty() {
        return Err(Error::CommandExecutionFailed(
            "No command provided".to_string(),
        ));
    }

    let program = &command[0];
    let args = &command[1..];

    // Print the command being executed
    let command_str = command.join(" ");
    println!("Executing command: {}", command_str);
    println!("In directory: {}", dir);
    println!();

    let output = Command::new(program)
        .args(args)
        .current_dir(dir)
        .output()
        .map_err(|e| {
            Error::CommandExecutionFailed(format!(
                "Failed to execute command '{}': {}",
                command_str,
                e
            ))
        })?;

    // Print stdout if not empty
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.is_empty() {
        println!("stdout:");
        println!("{}", stdout);
    }

    // Print stderr if not empty
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.is_empty() {
        println!("stderr:");
        println!("{}", stderr);
    }

    // Print exit status
    if let Some(code) = output.status.code() {
        println!("Exit code: {}", code);
    } else {
        println!("Process terminated by signal");
    }
    println!();

    if !output.status.success() {
        return Err(Error::CommandExecutionFailed(format!(
            "Command '{}' failed with exit code {}",
            command_str,
            output.status.code().unwrap_or(-1),
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_command_empty() {
        let result = execute_command(".", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_command_basic() {
        // Test with a simple command that should succeed
        let result = execute_command(".", &["echo".to_string(), "test".to_string()]);
        assert!(result.is_ok());
    }
}
