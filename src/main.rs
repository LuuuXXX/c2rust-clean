mod config_helper;
mod error;
mod executor;

use clap::{Args, Parser, Subcommand};
use error::Result;

#[derive(Parser)]
#[command(name = "c2rust-clean")]
#[command(about = "C project build artifact cleaning tool for c2rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute clean command and save configuration
    Clean(CommandArgs),
}

#[derive(Args)]
struct CommandArgs {
    /// Directory to execute clean command
    #[arg(long)]
    dir: Option<String>,

    /// Feature name (default: "default")
    #[arg(long)]
    feature: Option<String>,

    /// Clean command to execute (e.g., "make clean")
    #[arg(last = true)]
    command: Vec<String>,
}

fn run(args: CommandArgs) -> Result<()> {
    // 1. Check if c2rust-config exists
    config_helper::check_c2rust_config_exists()?;

    // 2. Read configuration from file
    let config = config_helper::read_config(args.feature.as_deref())?;

    // 3. Determine final values (CLI overrides config)
    let dir = args.dir.or(config.dir).ok_or_else(|| {
        error::Error::MissingParameter(
            "Directory not specified. Use --dir or set clean.dir in config".to_string(),
        )
    })?;

    let command = if !args.command.is_empty() {
        args.command
    } else if let Some(cmd_str) = config.command {
        // Parse command string into Vec<String>
        // Note: This uses simple whitespace splitting and doesn't handle quoted arguments.
        // For commands with quoted arguments, specify them directly on the CLI.
        cmd_str.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        return Err(error::Error::MissingParameter(
            "Command not specified. Provide command arguments or set clean in config".to_string(),
        ));
    };

    // 4. Execute the clean command
    executor::execute_command(&dir, &command)?;

    // 5. Save configuration using c2rust-config
    let command_str = command.join(" ");
    config_helper::save_config(&dir, &command_str, args.feature.as_deref())?;

    println!("Clean command executed successfully and configuration saved.");
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
