use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use sha2::{Sha256, Digest};
use crate::error::{CliError, Result};

#[derive(Debug, Clone)]
pub struct HashResult {
    pub sha256: String,
}

pub struct HashCalculator;

impl HashCalculator {
    pub fn calculate_file_hash<P: AsRef<Path>>(path: P) -> Result<String> {
        let path = path.as_ref();
        let mut file = File::open(path)
            .map_err(|e| CliError::FileOperation(e))?;

        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];

        loop {
            let bytes_read = file.read(&mut buffer)
                .map_err(|e| CliError::FileOperation(e))?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    pub fn calculate_string_hash(input: &str) -> HashResult {
        let hasher = Sha256::new();
        let sha256_result = format!("{:x}", hasher.chain_update(input.as_bytes()).finalize());

        HashResult {
            sha256: sha256_result,
        }
    }

    pub fn verify_file_hash<P: AsRef<Path>>(path: P, expected_hash: &str) -> Result<bool> {
        let actual_hash = Self::calculate_file_hash(path)?;
        Ok(actual_hash.to_lowercase() == expected_hash.to_lowercase())
    }

    pub fn calculate_directory_hash<P: AsRef<Path>>(path: P) -> Result<String> {
        let path = path.as_ref();
        let mut hasher = Sha256::new();

        let walker = walkdir::WalkDir::new(path)
            .sort_by(|a, b| a.file_name().cmp(b.file_name()));

        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let file_path = entry.path();
                let relative_path = file_path.strip_prefix(path)
                    .map_err(|_| CliError::InvalidInput("Invalid path".to_string()))?;

                // Add relative path to hash
                hasher.update(relative_path.to_string_lossy().as_bytes());
                hasher.update(b"\0");

                // Add file content to hash
                let file_hash = Self::calculate_file_hash(file_path)?;
                hasher.update(file_hash.as_bytes());
                hasher.update(b"\0");
            }
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    pub fn find_files_by_hash<P: AsRef<Path>>(
        root: P,
        target_hash: &str,
    ) -> Result<Vec<std::path::PathBuf>> {
        let root = root.as_ref();
        let mut matching_files = Vec::new();

        let walker = walkdir::WalkDir::new(root);

        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Ok(file_hash) = Self::calculate_file_hash(entry.path()) {
                    if file_hash.to_lowercase() == target_hash.to_lowercase() {
                        matching_files.push(entry.path().to_path_buf());
                    }
                }
            }
        }

        Ok(matching_files)
    }

    pub fn batch_calculate_hashes<P: AsRef<Path>>(
        paths: &[P],
    ) -> Result<Vec<(std::path::PathBuf, HashResult)>> {
        use rayon::prelude::*;

        paths
            .par_iter()
            .map(|path| {
                let path = path.as_ref();
                let hash_result = Self::calculate_string_hash(&format!("file: {}", path.display()))?;
                Ok::<_, CliError>((path.to_path_buf(), hash_result))
            })
            .collect()
    }

    pub fn compare_directories<P1: AsRef<Path>, P2: AsRef<Path>>(
        dir1: P1,
        dir2: P2,
    ) -> Result<DirectoryComparison> {
        let dir1 = dir1.as_ref();
        let dir2 = dir2.as_ref();

        let hash1 = Self::calculate_directory_hash(dir1)?;
        let hash2 = Self::calculate_directory_hash(dir2)?;

        let are_identical = hash1.to_lowercase() == hash2.to_lowercase();

        Ok(DirectoryComparison {
            dir1: dir1.to_path_buf(),
            dir2: dir2.to_path_buf(),
            hash1,
            hash2,
            are_identical,
        })
    }

    pub fn generate_checksum_file<P: AsRef<Path>>(
        directory: P,
        output_file: P,
    ) -> Result<()> {
        let directory = directory.as_ref();
        let output_file = output_file.as_ref();

        let mut content = String::new();

        let walker = walkdir::WalkDir::new(directory)
            .sort_by(|a, b| a.file_name().cmp(b.file_name()));

        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let file_path = entry.path();
                let relative_path = file_path.strip_prefix(directory)
                    .map_err(|_| CliError::InvalidInput("Invalid path".to_string()))?;

                let hash_result = Self::calculate_string_hash(&format!("file: {}", file_path.display()))?;
                content.push_str(&format!(
                    "{} {}\n",
                    hash_result.sha256,
                    relative_path.display()
                ));
            }
        }

        std::fs::write(output_file, content)
            .map_err(|e| CliError::FileOperation(e))?;

        Ok(())
    }

    pub fn verify_checksum_file<P: AsRef<Path>>(
        directory: P,
        checksum_file: P,
    ) -> Result<ChecksumVerification> {
        let directory = directory.as_ref();
        let checksum_file = checksum_file.as_ref();

        let content = std::fs::read_to_string(checksum_file)
            .map_err(|e| CliError::FileOperation(e))?;

        let mut verification = ChecksumVerification {
            total_files: 0,
            verified_files: 0,
            failed_files: Vec::new(),
        };

        for line in content.lines() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }

            let expected_hash = parts[0];
            let file_path = parts[1..].join(" ");

            let full_path = directory.join(&file_path);

            verification.total_files += 1;

            if let Ok(actual_hash) = Self::calculate_file_hash(&full_path) {
                if actual_hash.to_lowercase() == expected_hash.to_lowercase() {
                    verification.verified_files += 1;
                } else {
                    verification.failed_files.push((file_path, format!(
                        "Hash mismatch. Expected: {}, Actual: {}",
                        expected_hash,
                        actual_hash
                    )));
                }
            } else {
                verification.failed_files.push((file_path, "Failed to calculate file hash".to_string()));
            }
        }

        Ok(verification)
    }
}

#[derive(Debug, Clone)]
pub struct DirectoryComparison {
    pub dir1: std::path::PathBuf,
    pub dir2: std::path::PathBuf,
    pub hash1: String,
    pub hash2: String,
    pub are_identical: bool,
}

#[derive(Debug, Clone)]
pub struct ChecksumVerification {
    pub total_files: usize,
    pub verified_files: usize,
    pub failed_files: Vec<(String, String)>,
}

impl DirectoryComparison {
    pub fn print_result(&self) {
        println!("Directory Comparison Results:");
        println!("Directory 1: {}", self.dir1.display());
        println!("Directory 2: {}", self.dir2.display());
        println!("Hash 1: {}", self.hash1);
        println!("Hash 2: {}", self.hash2);

        if self.are_identical {
            println!("✓ Directories are identical");
        } else {
            println!("✗ Directories differ");
        }
    }
}

impl ChecksumVerification {
    pub fn is_successful(&self) -> bool {
        self.failed_files.is_empty()
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_files == 0 {
            0.0
        } else {
            (self.verified_files as f64 / self.total_files as f64) * 100.0
        }
    }

    pub fn print_results(&self) {
        println!("Checksum Verification Results:");
        println!("Total files: {}", self.total_files);
        println!("Verified files: {}", self.verified_files);
        println!("Success rate: {:.1}%", self.success_rate());

        if !self.failed_files.is_empty() {
            println!("Failed files:");
            for (file, error) in &self.failed_files {
                println!("  {}: {}", file, error);
            }
        }
    }
}
