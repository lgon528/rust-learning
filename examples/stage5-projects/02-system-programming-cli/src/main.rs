mod commands;
mod config;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

use commands::{FileArgs, FileCommandExecutor};

#[derive(Parser)]
#[command(
    name = "system-programming-cli",
    about = "A comprehensive system programming CLI tool",
    version = "0.1.0",
    author = "Rust Learning Project",
)]
struct Cli {
    /// Show configuration
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// File operations and analysis
    File {
        #[command(flatten)]
        args: FileArgs,
    },

    /// Show configuration
    Config {
        /// Show current configuration
        #[arg(short, long)]
        show: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("🦀 Starting system-programming-cli");

    match cli.command {
        Commands::File { args } => {
            FileCommandExecutor::execute(args)?;
        }
        Commands::Config { show } => {
            if show {
                println!("Current configuration:");
                println!("(Default implementation)");
            }
        }
    }

    Ok(())
}
