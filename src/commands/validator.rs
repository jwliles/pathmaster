//! Path validation functionality for the pathmaster tool.
//!
//! This module provides functionality to validate directories in the PATH
//! environment variable, separating them into existing and missing directories.
//! It handles validation of both individual paths and the complete PATH.

use std::env;
use std::path::{Path, PathBuf};

/// Represents the validation results of PATH directories.
#[derive(Debug, PartialEq)]
pub struct PathValidation {
    /// Directories that exist in the filesystem
    pub existing_dirs: Vec<PathBuf>,
    /// Directories that are in PATH but don't exist
    pub missing_dirs: Vec<PathBuf>,
}

/// Validates whether a path is a valid directory for PATH inclusion.
///
/// # Arguments
/// * `path` - The path to validate
///
/// # Returns
/// * `true` if the path exists and is a directory
/// * `false` otherwise
pub fn is_valid_path_entry(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

impl PathValidation {
    /// Creates a new empty PathValidation instance.
    pub fn new() -> Self {
        PathValidation {
            existing_dirs: Vec::new(),
            missing_dirs: Vec::new(),
        }
    }

    /// Adds a path to the appropriate list based on its validity.
    ///
    /// # Arguments
    /// * `path` - The path to validate and add
    pub fn add_path(&mut self, path: PathBuf) {
        if is_valid_path_entry(&path) {
            self.existing_dirs.push(path);
        } else {
            self.missing_dirs.push(path);
        }
    }

    /// Returns the total number of directories (both valid and invalid).
    #[allow(dead_code)]
    pub fn total_dirs(&self) -> usize {
        self.existing_dirs.len() + self.missing_dirs.len()
    }
}

/// Validates all directories in the current PATH environment variable.
///
/// # Returns
/// * `Ok(PathValidation)` - Validation results with existing and missing directories
/// * `Err(std::io::Error)` - If there are problems accessing the filesystem
pub fn validate_path() -> std::io::Result<PathValidation> {
    let mut validation = PathValidation::new();

    // Get PATH entries, return empty validation if PATH is unset or empty
    let path_var = match env::var_os("PATH") {
        Some(path) => {
            let path_str = path.to_string_lossy();
            if path_str.trim().is_empty() {
                return Ok(validation);
            }
            path
        }
        None => return Ok(validation),
    };

    // Process each PATH entry
    for entry in env::split_paths(&path_var) {
        if !entry.as_os_str().is_empty() {
            validation.add_path(entry);
        }
    }

    // Sort for consistent output
    validation.existing_dirs.sort();
    validation.missing_dirs.sort();

    Ok(validation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_path_validation() {
        let temp_dir = TempDir::new().unwrap();
        let valid_path = temp_dir.path().to_owned();
        let invalid_path = temp_dir.path().join("nonexistent");

        assert!(is_valid_path_entry(&valid_path));
        assert!(!is_valid_path_entry(&invalid_path));
    }

    #[test]
    fn test_validation_struct() {
        let mut validation = PathValidation::new();
        let temp_dir = TempDir::new().unwrap();

        // Test with valid directory
        validation.add_path(temp_dir.path().to_owned());
        assert_eq!(validation.existing_dirs.len(), 1);
        assert_eq!(validation.missing_dirs.len(), 0);

        // Test with invalid directory
        validation.add_path(temp_dir.path().join("nonexistent"));
        assert_eq!(validation.existing_dirs.len(), 1);
        assert_eq!(validation.missing_dirs.len(), 1);
    }

    #[test]
    fn test_total_dirs() {
        let mut validation = PathValidation::new();
        assert_eq!(validation.total_dirs(), 0);

        validation.existing_dirs.push(PathBuf::from("/valid"));
        validation.missing_dirs.push(PathBuf::from("/invalid"));
        assert_eq!(validation.total_dirs(), 2);
    }
}
