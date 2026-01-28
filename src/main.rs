mod error;
mod executor;

use clap::{Args, Parser, Subcommand};
use error::Result;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "c2rust-clean")]
#[command(about = "C project build artifact cleaning tool for c2rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute clean command
    Clean(CommandArgs),
}

#[derive(Args)]
struct CommandArgs {
    /// Clean command to execute - use after '--' separator
    /// Example: c2rust-clean clean -- make clean
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true, value_name = "CLEAN_CMD")]
    clean_cmd: Vec<String>,
}

/// Find the project root directory by searching for .c2rust directory
/// or return the current directory as the root.
fn find_project_root(start_dir: &Path) -> Result<PathBuf> {
    let mut current = start_dir;
    loop {
        let c2rust_dir = current.join(".c2rust");
        if c2rust_dir.exists() && c2rust_dir.is_dir() {
            return Ok(current.to_path_buf());
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => return Ok(start_dir.to_path_buf()),
        }
    }
}

fn run(args: CommandArgs) -> Result<()> {
    // 1. Get the current working directory (where the command is executed)
    let current_dir = std::env::current_dir()
        .map_err(|e| error::Error::CommandExecutionFailed(
            format!("Failed to get current directory: {}", e)
        ))?;
    
    // 2. Find the project root (where .c2rust will be created)
    // Start from current directory and search upward for .c2rust or use current as root
    let project_root = find_project_root(&current_dir)?;
    
    // 3. Calculate the clean directory relative to project root
    let clean_dir_relative = current_dir.strip_prefix(&project_root)
        .map(|p| {
            if p.as_os_str().is_empty() {
                ".".to_string()
            } else {
                p.display().to_string()
            }
        })
        .unwrap_or_else(|_| {
            eprintln!("Warning: current directory is not under project root, using '.' as clean directory");
            ".".to_string()
        });

    // Print the calculated paths for debugging
    println!("Project root: {}", project_root.display());
    println!("Current directory: {}", current_dir.display());
    println!("Relative clean directory: {}", clean_dir_relative);
    println!();

    // Execute the clean command in the current directory
    executor::execute_command(&current_dir, &args.clean_cmd)?;

    println!("Clean command executed successfully.");
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Clean(args) => run(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
