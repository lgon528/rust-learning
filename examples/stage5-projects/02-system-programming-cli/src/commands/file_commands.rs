use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Args)]
pub struct FileArgs {
    #[command(subcommand)]
    pub command: FileCommands,
}

#[derive(Subcommand)]
pub enum FileCommands {
    /// Count lines in files
    Count {
        /// File(s) to count
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },

    /// Show directory tree
    Tree {
        /// Directory to display
        #[arg(default_value = ".")]
        directory: String,
    },
}

pub struct FileCommandExecutor;

impl FileCommandExecutor {
    pub fn execute(args: FileArgs) -> Result<(), Box<dyn std::error::Error>> {
        match args.command {
            FileCommands::Count { files } => {
                Self::count_files(files)
            }
            FileCommands::Tree { directory } => {
                Self::show_tree(directory)
            }
        }
    }

    fn count_files(files: Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
        println!("Counting lines in {} files...", files.len());

        for file in &files {
            let content = std::fs::read_to_string(file)?;
            let line_count = content.lines().count();
            println!("{}: {} lines", file.display(), line_count);
        }

        Ok(())
    }

    fn show_tree(directory: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("Directory tree for: {}", directory);
        println!("(Simplified implementation)");

        let entries: Vec<_> = std::fs::read_dir(&directory)?
            .filter_map(Result::ok)
            .collect();

        for entry in entries {
            let path = entry.path();
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            if path.is_file() {
                println!("├── {}", name);
            } else if path.is_dir() {
                println!("├── {}/", name);
            }
        }

        Ok(())
    }
}
