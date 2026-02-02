mod config_helper;
mod error;
mod executor;
mod git_helper;

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
    /// Optional feature name (default: "default")
    #[arg(long)]
    feature: Option<String>,

    /// Clean command to execute - use after '--' separator
    /// Example: c2rust-clean clean -- make clean
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true, value_name = "CLEAN_CMD")]
    clean_cmd: Vec<String>,
}

/// Find the project root directory by searching for marker files/directories.
/// Searches upward from start_dir for directories containing:
/// - .git directory (Git repository root)
/// - Cargo.toml (Rust project root)
/// - .c2rust directory (c2rust project marker)
/// If none found, returns the start_dir as root.
fn find_project_root(start_dir: &Path) -> Result<PathBuf> {
    let mut current = start_dir;
    
    // List of marker files/directories that indicate a project root
    let markers = [".git", "Cargo.toml", ".c2rust"];
    
    loop {
        // Check if any marker exists in the current directory
        for marker in &markers {
            let marker_path = current.join(marker);
            if marker_path.exists() {
                return Ok(current.to_path_buf());
            }
        }
        
        // Move to parent directory
        match current.parent() {
            Some(parent) => current = parent,
            None => return Ok(start_dir.to_path_buf()),
        }
    }
}

fn run(args: CommandArgs) -> Result<()> {
    // 1. Check if c2rust-config exists
    config_helper::check_c2rust_config_exists()?;

    // 2. Get feature name (default to "default")
    let feature = args.feature.as_deref().unwrap_or("default");

    // 3. Get the current working directory (where the command is executed)
    let current_dir = std::env::current_dir()?;
    
    // 4. Find the project root (where .c2rust is located)
    // Start from current directory and search upward for .c2rust or use current as root
    let project_root = find_project_root(&current_dir)?;
    
    // 5. Calculate the clean directory relative to project root
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

    // Print the calculated paths to stderr for debugging
    eprintln!("Project root: {}", project_root.display());
    eprintln!("Current directory: {}", current_dir.display());
    eprintln!("Relative clean directory: {}", clean_dir_relative);
    eprintln!();

    // Execute the clean command in the current directory
    executor::execute_command(&current_dir, &args.clean_cmd)?;

    // Save configuration using c2rust-config
    let command_str = args.clean_cmd.join(" ");
    config_helper::save_config(&clean_dir_relative, &command_str, Some(feature), &project_root)?;

    // Auto-commit changes in .c2rust directory if any
    git_helper::auto_commit_if_modified(&project_root)?;

    println!("\n✓ Clean command executed successfully.");
    println!("✓ Configuration saved.");
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
