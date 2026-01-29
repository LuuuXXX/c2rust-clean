mod config_helper;
mod error;
mod executor;
mod git_helper;

use clap::{Args, Parser, Subcommand};
use error::Result;
use log::info;

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

fn run(args: CommandArgs) -> Result<()> {
    // 1. Check if c2rust-config exists
    config_helper::check_c2rust_config_exists()?;

    // 2. Get feature name (default to "default")
    let feature = args.feature.as_deref().unwrap_or("default");

    // 3. Get the current working directory (where the command is executed)
    let current_dir = std::env::current_dir()?;
    
    // 4. Find the project root using git_helper (supports C2RUST_PROJECT_ROOT)
    let project_root = git_helper::get_project_root(&current_dir)?;
    
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

    // Auto-commit changes to .c2rust if any (unless disabled)
    let auto_commit_disabled = std::env::var("C2RUST_DISABLE_AUTO_COMMIT")
        .map(|v| {
            let v_lower = v.to_lowercase();
            !v.is_empty() && v_lower != "0" && v_lower != "false"
        })
        .unwrap_or(false);
    
    if !auto_commit_disabled {
        info!("Checking for .c2rust directory changes to auto-commit");
        if let Err(e) = git_helper::auto_commit_c2rust_changes(&project_root) {
            eprintln!("Warning: Failed to auto-commit .c2rust changes: {}", e);
        }
    } else {
        info!("Auto-commit disabled via C2RUST_DISABLE_AUTO_COMMIT environment variable");
    }

    println!("\n✓ Clean command executed successfully.");
    println!("✓ Configuration saved.");
    Ok(())
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Clean(args) => run(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
