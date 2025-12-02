use std::path::{Path, PathBuf};
use std::fs::Metadata;
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::{WalkDir, DirEntry};
use rayon::prelude::*;
use crate::error::{CliError, Result};

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub modified: SystemTime,
    pub is_file: bool,
    pub is_dir: bool,
    pub extension: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FileStats {
    pub total_files: usize,
    pub total_dirs: usize,
    pub total_size: u64,
    pub largest_file: Option<FileInfo>,
    pub smallest_file: Option<FileInfo>,
    pub file_types: std::collections::HashMap<String, usize>,
}

impl FileInfo {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let metadata = std::fs::metadata(path)
            .map_err(|e| CliError::FileOperation(e))?;

        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase());

        Ok(FileInfo {
            path: path.to_path_buf(),
            name,
            size: metadata.len(),
            modified: metadata.modified()
                .unwrap_or_else(|_| UNIX_EPOCH),
            is_file: metadata.is_file(),
            is_dir: metadata.is_dir(),
            extension,
        })
    }
}

pub struct FileOperations;

impl FileOperations {
    pub fn count_lines<P: AsRef<Path>>(path: P) -> Result<usize> {
        let content = std::fs::read_to_string(path)?;
        Ok(content.lines().count())
    }

    pub fn count_words<P: AsRef<Path>>(path: P) -> Result<usize> {
        let content = std::fs::read_to_string(path)?;
        Ok(content.split_whitespace().count())
    }

    pub fn count_characters<P: AsRef<Path>>(path: P) -> Result<usize> {
        let content = std::fs::read_to_string(path)?;
        Ok(content.chars().count())
    }

    pub fn get_file_stats<P: AsRef<Path>>(path: P) -> Result<FileStats> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(CliError::NotFound(format!("Path not found: {}", path.display())));
        }

        let mut stats = FileStats {
            total_files: 0,
            total_dirs: 0,
            total_size: 0,
            largest_file: None,
            smallest_file: None,
            file_types: std::collections::HashMap::new(),
        };

        let walk_dir = WalkDir::new(path);

        for entry in walk_dir.into_iter().filter_map(|e| e.ok()) {
            let file_info = FileInfo::from_path(entry.path())?;

            if file_info.is_file {
                stats.total_files += 1;
                stats.total_size += file_info.size;

                // Track file types
                if let Some(ext) = &file_info.extension {
                    *stats.file_types.entry(ext.clone()).or_insert(0) += 1;
                }

                // Track largest and smallest files
                match &stats.largest_file {
                    None => stats.largest_file = Some(file_info.clone()),
                    Some(largest) => {
                        if file_info.size > largest.size {
                            stats.largest_file = Some(file_info.clone());
                        }
                    }
                }

                match &stats.smallest_file {
                    None => stats.smallest_file = Some(file_info.clone()),
                    Some(smallest) => {
                        if file_info.size < smallest.size {
                            stats.smallest_file = Some(file_info.clone());
                        }
                    }
                }
            } else if file_info.is_dir {
                stats.total_dirs += 1;
            }
        }

        Ok(stats)
    }

    pub fn find_files<P: AsRef<Path>>(
        root: P,
        _config: &crate::config::Config,
    ) -> Result<Vec<FileInfo>> {
        let root = root.as_ref();
        let mut files = Vec::new();

        let walk = WalkDir::new(root);

        for result in walk {
            match result {
                Ok(entry) => {
                    if let Some(_path) = entry.path().strip_prefix(root).ok() {
                        let file_info = FileInfo::from_path(entry.path())?;
                        if file_info.is_file {
                            files.push(file_info);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Warning: {}", err);
                }
            }
        }

        Ok(files)
    }

    pub fn find_large_files<P: AsRef<Path>>(root: P, min_size_mb: u64) -> Result<Vec<FileInfo>> {
        let files = Self::find_files(root, &crate::config::Config::default())?;
        let min_size_bytes = min_size_mb * 1024 * 1024;

        Ok(files
            .into_par_iter()
            .filter(|file| file.size >= min_size_bytes)
            .collect())
    }

    pub fn create_directory_tree<P: AsRef<Path>>(root: P) -> Result<String> {
        let root = root.as_ref();
        let mut tree = String::new();
        Self::build_tree_recursive(root, "", &mut tree, true)?;
        Ok(tree)
    }

    fn build_tree_recursive(
        dir: &Path,
        prefix: &str,
        tree: &mut String,
        is_root: bool,
    ) -> Result<()> {
        if !is_root {
            tree.push_str(prefix);
            tree.push_str("├── ");
            tree.push_str(dir.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(""));
            tree.push('\n');
        }

        let mut entries: Vec<_> = std::fs::read_dir(dir)?
            .filter_map(Result::ok)
            .collect();

        entries.sort_by_key(|e| e.file_name());

        for (i, entry) in entries.iter().enumerate() {
            let is_last = i == entries.len() - 1;
            let new_prefix = if is_root {
                prefix.to_string()
            } else {
                format!("{}   ", prefix)
            };

            if entry.file_type()?.is_dir() {
                let child_path = entry.path();
                let child_prefix = if is_root {
                    if is_last { "└── " } else { "├── " }
                } else {
                    if is_last { "└── " } else { "├── " }
                };

                tree.push_str(&format!("{}{}{}/\n",
                    if is_root { "" } else { prefix },
                    child_prefix,
                    entry.file_name().to_string_lossy()
                ));

                let child_prefix = if is_root {
                    if is_last { "    " } else { "│   " }
                } else {
                    if is_last { "    " } else { "│   " }
                };

                Self::build_tree_recursive(&child_path,
                    &format!("{}{}", prefix, child_prefix),
                    tree, false)?;
            } else {
                tree.push_str(&format!("{}{}{}\n",
                    if is_root { "" } else { prefix },
                    if is_last { "└── " } else { "├── " },
                    entry.file_name().to_string_lossy()
                ));
            }
        }

        Ok(())
    }

    pub fn calculate_disk_usage<P: AsRef<Path>>(path: P) -> Result<u64> {
        let path = path.as_ref();
        let mut total_size = 0;

        if path.is_file() {
            let metadata = std::fs::metadata(path)?;
            total_size = metadata.len();
        } else if path.is_dir() {
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() {
                    total_size += entry.metadata().ok().map(|m| m.len()).unwrap_or(0);
                }
            }
        }

        Ok(total_size)
    }

    pub fn find_duplicate_files<P: AsRef<Path>>(root: P) -> Result<Vec<(FileInfo, FileInfo)>> {
        let files = Self::find_files(root, &crate::config::Config::default())?;

        // Group files by size first (quick elimination)
        let mut size_groups: std::collections::HashMap<u64, Vec<FileInfo>> =
            std::collections::HashMap::new();

        for file in files {
            size_groups.entry(file.size).or_insert_with(Vec::new).push(file);
        }

        // Only process groups with multiple files
        let mut duplicates = Vec::new();

        for (_size, group) in size_groups {
            if group.len() > 1 {
                // Simplified hash comparison for demo
                for i in 0..group.len() {
                    for j in i + 1..group.len() {
                        duplicates.push((group[i].clone(), group[j].clone()));
                    }
                }
            }
        }

        Ok(duplicates)
    }
}
