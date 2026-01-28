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
    /// Execute clean command
    Clean(CommandArgs),
}

#[derive(Args)]
struct CommandArgs {
    /// Directory to execute clean command (required)
    #[arg(long = "dir", required = true)]
    dir: String,

    /// Clean command to execute (required, can be multiple arguments)
    #[arg(long = "cmd", required = true, num_args = 1.., allow_hyphen_values = true)]
    cmd: Vec<String>,
}

fn run(args: CommandArgs) -> Result<()> {
    // Validate that the directory exists
    let dir_path = std::path::Path::new(&args.dir);
    if !dir_path.exists() {
        return Err(error::Error::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory does not exist: {}", args.dir),
        )));
    }
    
    if !dir_path.is_dir() {
        return Err(error::Error::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Path is not a directory: {}", args.dir),
        )));
    }

    // Execute the clean command
    executor::execute_command(&args.dir, &args.cmd)?;

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
