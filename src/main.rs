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
    #[arg(long = "test.dir", required = true)]
    test_dir: String,

    /// Clean command to execute (required, can be multiple arguments)
    #[arg(long = "test.cmd", required = true, num_args = 1.., allow_hyphen_values = true)]
    test_cmd: Vec<String>,
}

fn run(args: CommandArgs) -> Result<()> {
    // Validate that the directory exists
    let dir_path = std::path::Path::new(&args.test_dir);
    if !dir_path.exists() {
        return Err(error::Error::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory does not exist: {}", args.test_dir),
        )));
    }
    
    if !dir_path.is_dir() {
        return Err(error::Error::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Path is not a directory: {}", args.test_dir),
        )));
    }

    // Execute the clean command
    executor::execute_command(&args.test_dir, &args.test_cmd)?;

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
