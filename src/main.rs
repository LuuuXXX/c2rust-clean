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
    /// Execute build command
    Build(CommandArgs),
}

#[derive(Args)]
struct CommandArgs {
    /// Directory to execute build command (required)
    #[arg(long = "build.dir", required = true)]
    build_dir: String,

    /// Build command to execute (required, can be multiple arguments)
    #[arg(long = "build.cmd", required = true, num_args = 1.., allow_hyphen_values = true)]
    build_cmd: Vec<String>,
}

fn run(args: CommandArgs) -> Result<()> {
    // Validate that the directory exists
    let dir_path = std::path::Path::new(&args.build_dir);
    if !dir_path.exists() {
        return Err(error::Error::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory does not exist: {}", args.build_dir),
        )));
    }
    
    if !dir_path.is_dir() {
        return Err(error::Error::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Path is not a directory: {}", args.build_dir),
        )));
    }

    // Execute the build command
    executor::execute_command(&args.build_dir, &args.build_cmd)?;

    println!("Build command executed successfully.");
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Build(args) => run(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
