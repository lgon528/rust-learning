use clap::{Args, Subcommand};
use std::path::PathBuf;
use crate::error::Result;
use crate::utils::{
    HashCalculator, ProgressManager, TablePrinter, ColorPrinter,
};

#[derive(Args)]
pub struct HashArgs {
    #[command(subcommand)]
    pub command: HashCommands,
}

#[derive(Subcommand)]
pub enum HashCommands {
    /// Calculate hash for a file
    File {
        /// File to hash
        #[arg(required = true)]
        file: PathBuf,
    },

    /// Calculate hash for a string
    String {
        /// String to hash
        #[arg(required = true)]
        input: String,
    },

    /// Calculate hash for a directory
    Directory {
        /// Directory to hash
        #[arg(required = true)]
        directory: PathBuf,

        /// Show progress bar
        #[arg(short, long)]
        progress: bool,
    },

    /// Verify file hash
    Verify {
        /// File to verify
        #[arg(required = true)]
        file: PathBuf,

        /// Expected hash
        #[arg(required = true)]
        expected_hash: String,
    },
}

pub struct HashCommandExecutor;

impl HashCommandExecutor {
    pub fn execute(args: HashArgs) -> Result<()> {
        match args.command {
            HashCommands::File { file } => {
                Self::hash_file(file)
            }
            HashCommands::String { input } => {
                Self::hash_string(input)
            }
            HashCommands::Directory { directory, progress } => {
                Self::hash_directory(directory, progress)
            }
            HashCommands::Verify { file, expected_hash } => {
                Self::verify_file_hash(file, expected_hash)
            }
        }
    }

    fn hash_file(file: PathBuf) -> Result<()> {
        let file_hash = HashCalculator::calculate_file_hash(&file)?;

        println!("File: {}", file.display());
        println!("Algorithm: SHA256");
        println!("Hash: {}", file_hash);

        Ok(())
    }

    fn hash_string(input: String) -> Result<()> {
        let hash_result = HashCalculator::calculate_string_hash(&input);

        println!("Input: {}", input);
        println!("Algorithm: SHA256");
        println!("Hash: {}", hash_result.sha256);

        Ok(())
    }

    fn hash_directory(directory: PathBuf, progress: bool) -> Result<()> {
        let _bar = if progress {
            Some(ProgressManager::new_bar(0, "Calculating directory hash"))
        } else {
            None
        };

        let dir_hash = HashCalculator::calculate_directory_hash(&directory)?;

        if let Some(bar) = _bar {
            bar.finish();
        }

        println!("Directory: {}", directory.display());
        println!("Hash: {}", dir_hash);

        Ok(())
    }

    fn verify_file_hash(file: PathBuf, expected_hash: String) -> Result<()> {
        let is_valid = HashCalculator::verify_file_hash(&file, &expected_hash)?;

        println!("File: {}", file.display());
        println!("Algorithm: SHA256");
        println!("Expected: {}", expected_hash);
        println!("Result: {}",
            if is_valid {
                ColorPrinter::green("✓ VALID")
            } else {
                ColorPrinter::red("✗ INVALID")
            }
        );

        Ok(())
    }
}
