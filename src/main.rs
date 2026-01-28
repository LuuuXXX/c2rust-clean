mod config;
mod error;
mod executor;

use clap::{Args, Parser, Subcommand};
use config::Config;
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
    /// Clean command to execute (required)
    /// Use -- to separate c2rust-clean arguments from command arguments
    /// Example: c2rust-clean clean --cmd make -- clean all
    #[arg(long = "cmd", required = true, num_args = 1.., allow_hyphen_values = true)]
    cmd: Vec<String>,
}

fn run(args: CommandArgs) -> Result<()> {
    // Find the .c2rust root directory
    let c2rust_root = Config::find_c2rust_root()?;
    
    // Calculate and save the current directory relative to .c2rust root
    let relative_dir = Config::calculate_relative_dir(&c2rust_root)?;
    Config::save(&c2rust_root, &relative_dir)?;
    
    // Get the build directory path
    let config = Config::load(&c2rust_root)?;
    let build_dir = Config::get_build_dir_path(&c2rust_root, &config);
    
    // Validate that the build directory exists
    if !build_dir.exists() {
        return Err(error::Error::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Build directory does not exist: {}", build_dir.display()),
        )));
    }
    
    if !build_dir.is_dir() {
        return Err(error::Error::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Build path is not a directory: {}", build_dir.display()),
        )));
    }

    // Convert build_dir to string for executor
    let build_dir_str = build_dir.to_str().ok_or_else(|| {
        error::Error::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Build directory path contains invalid UTF-8",
        ))
    })?;

    // Execute the clean command
    executor::execute_command(build_dir_str, &args.cmd)?;

    println!("Clean command executed successfully.");
    Ok(())
}

fn main() {
    // Manual argument parsing to handle -- separator
    let args: Vec<String> = std::env::args().collect();
    
    // Find the position of --
    let separator_pos = args.iter().position(|arg| arg == "--");
    
    // Split arguments at the -- separator
    let (c2rust_args, cmd_extra_args) = if let Some(pos) = separator_pos {
        (&args[..pos], &args[pos + 1..])
    } else {
        (args.as_slice(), &[][..])
    };
    
    // Parse c2rust-clean arguments
    let cli = match Cli::try_parse_from(c2rust_args) {
        Ok(cli) => cli,
        Err(e) => {
            // Check if this is a help or version request (exit code 0)
            if e.exit_code() == 0 {
                print!("{}", e);
                std::process::exit(0);
            } else {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    };

    let result = match cli.command {
        Commands::Clean(mut cmd_args) => {
            // Append arguments after -- to the command
            cmd_args.cmd.extend(cmd_extra_args.iter().cloned());
            run(cmd_args)
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
