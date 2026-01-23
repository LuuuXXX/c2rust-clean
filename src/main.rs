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
    Clean(CleanArgs),
}

#[derive(Args)]
struct CleanArgs {
    /// Directory to execute clean command
    #[arg(long, required = true)]
    dir: String,

    /// Feature name (default: "default")
    #[arg(long)]
    feature: Option<String>,

    /// Clean command to execute (e.g., "make clean")
    #[arg(last = true, required = true)]
    command: Vec<String>,
}

fn run(args: CleanArgs) -> Result<()> {
    // 1. Check if c2rust-config exists
    config_helper::check_c2rust_config_exists()?;

    // 2. Execute the clean command
    executor::execute_command(&args.dir, &args.command)?;

    // 3. Save configuration using c2rust-config
    let command_str = args.command.join(" ");
    config_helper::save_config(&args.dir, &command_str, args.feature.as_deref())?;

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
