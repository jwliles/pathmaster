//! Path validation functionality for the pathmaster tool.
//!
//! This module provides functionality to validate directories in the PATH
//! environment variable, separating them into existing and missing directories.

use crate::utils;
use std::env;
use std::path::PathBuf;

/// Represents the validation results of PATH directories.
#[derive(Debug, PartialEq)]
pub struct PathValidation {
    /// Directories that exist in the filesystem
    pub existing_dirs: Vec<PathBuf>,
    /// Directories that are in PATH but don't exist
    pub missing_dirs: Vec<PathBuf>,
}

impl PathValidation {
    /// Creates a new empty PathValidation instance.
    ///
    /// Returns a PathValidation with empty vectors for both existing and missing directories.
    fn new() -> Self {
        PathValidation {
            existing_dirs: Vec::new(),
            missing_dirs: Vec::new(),
        }
    }
}

/// Validates all directories in the current PATH environment variable.
///
/// # Returns
///
/// Returns a Result containing PathValidation which separates directories into:
/// - existing_dirs: Directories that exist in the filesystem
/// - missing_dirs: Directories that are in PATH but don't exist
///
/// Returns empty vectors if PATH is empty or unset.
///
/// # Errors
///
/// Returns an IO error if there are problems accessing the filesystem.
///
/// # Example
///
/// ```
/// use path_finder::commands::validator;
///
/// match validator::validate_path() {
///     Ok(validation) => {
///         if !validation.missing_dirs.is_empty() {
///             println!("Found {} invalid directories in PATH", validation.missing_dirs.len());
///         }
///     },
///     Err(e) => eprintln!("Error validating PATH: {}", e),
/// }
/// ```
pub fn validate_path() -> std::io::Result<PathValidation> {
    let mut validation = PathValidation::new();

    // Get PATH entries, handle empty PATH case
    let path_var = match env::var("PATH") {
        Ok(path) if !path.is_empty() => path,
        _ => return Ok(validation), // Return empty validation for empty PATH
    };

    // Process only if PATH is not empty
    for entry in env::split_paths(&path_var) {
        let expanded_path = utils::expand_path(&entry.to_string_lossy());

        if expanded_path.exists() && expanded_path.is_dir() {
            validation.existing_dirs.push(expanded_path);
        } else {
            validation.missing_dirs.push(expanded_path);
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
    use std::fs;
    use tempfile::TempDir;

    /// Helper function to run tests with a specific PATH value and ensure cleanup.
    ///
    /// # Arguments
    ///
    /// * `test_path` - The PATH value to use during the test
    /// * `test` - The test function to run
    ///
    /// This function will:
    /// 1. Save the current PATH
    /// 2. Set the specified test PATH
    /// 3. Run the test
    /// 4. Restore the original PATH, even if the test panics
    fn with_test_path<F>(test_path: &str, test: F)
    where
        F: FnOnce() + std::panic::UnwindSafe,
    {
        // Save original PATH
        let original_path = env::var("PATH").ok();

        // Set test PATH
        env::set_var("PATH", test_path);

        // Run test, ensuring PATH is restored even if test panics
        let result = std::panic::catch_unwind(test);

        // Restore original PATH
        match original_path {
            Some(path) => env::set_var("PATH", path),
            None => env::remove_var("PATH"),
        }

        // Re-panic if test panicked
        if let Err(err) = result {
            std::panic::resume_unwind(err);
        }
    }

    #[test]
    fn test_validate_path() {
        let temp_dir = TempDir::new().unwrap();
        let valid_path = temp_dir.path().join("valid");
        let invalid_path = temp_dir.path().join("invalid");

        fs::create_dir(&valid_path).unwrap();

        let test_path = format!(
            "{}:{}",
            valid_path.to_string_lossy(),
            invalid_path.to_string_lossy()
        );

        with_test_path(&test_path, || {
            let validation = validate_path().unwrap();
            assert_eq!(validation.existing_dirs.len(), 1);
            assert_eq!(validation.missing_dirs.len(), 1);
            assert!(validation.existing_dirs.contains(&valid_path));
            assert!(validation.missing_dirs.contains(&invalid_path));
        });
    }

    #[test]
    fn test_validate_path_empty_path() {
        with_test_path("", || {
            let validation = validate_path().unwrap();
            assert!(
                validation.existing_dirs.is_empty(),
                "Expected no existing directories with empty PATH"
            );
            assert!(
                validation.missing_dirs.is_empty(),
                "Expected no missing directories with empty PATH"
            );
        });
    }

    #[test]
    fn test_validate_unset_path() {
        let original_path = env::var("PATH").ok();
        env::remove_var("PATH");

        let validation = validate_path().unwrap();
        assert!(
            validation.existing_dirs.is_empty(),
            "Expected no existing directories with unset PATH"
        );
        assert!(
            validation.missing_dirs.is_empty(),
            "Expected no missing directories with unset PATH"
        );

        if let Some(path) = original_path {
            env::set_var("PATH", path);
        }
    }

    #[test]
    fn test_validation_sorting() {
        let temp_dir = TempDir::new().unwrap();
        let path_b = temp_dir.path().join("b");
        let path_a = temp_dir.path().join("a");
        let path_c = temp_dir.path().join("c");

        let test_path = format!(
            "{}:{}:{}",
            path_b.to_string_lossy(),
            path_a.to_string_lossy(),
            path_c.to_string_lossy()
        );

        with_test_path(&test_path, || {
            let validation = validate_path().unwrap();
            assert_eq!(validation.missing_dirs.len(), 3);
            assert!(validation.missing_dirs[0] < validation.missing_dirs[1]);
            assert!(validation.missing_dirs[1] < validation.missing_dirs[2]);
        });
    }

    #[test]
    fn test_existing_and_missing_mix() {
        let temp_dir = TempDir::new().unwrap();
        let existing_a = temp_dir.path().join("existing_a");
        let existing_b = temp_dir.path().join("existing_b");
        let missing_a = temp_dir.path().join("missing_a");
        let missing_b = temp_dir.path().join("missing_b");

        fs::create_dir(&existing_a).unwrap();
        fs::create_dir(&existing_b).unwrap();

        let test_path = format!(
            "{}:{}:{}:{}",
            existing_b.to_string_lossy(),
            missing_a.to_string_lossy(),
            existing_a.to_string_lossy(),
            missing_b.to_string_lossy()
        );

        with_test_path(&test_path, || {
            let validation = validate_path().unwrap();
            assert_eq!(validation.existing_dirs.len(), 2);
            assert_eq!(validation.missing_dirs.len(), 2);
            assert!(validation.existing_dirs[0] < validation.existing_dirs[1]);
            assert!(validation.missing_dirs[0] < validation.missing_dirs[1]);
        });
    }
}
