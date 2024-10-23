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
/// Returns PathValidation with empty vectors if PATH is:
/// - Unset (environment variable doesn't exist)
/// - Empty (environment variable exists but is empty)
/// - Contains only whitespace
///
/// # Errors
///
/// Returns an IO error if there are problems accessing the filesystem.
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

    // Split PATH and process each entry
    for entry in env::split_paths(&path_var) {
        if entry.as_os_str().is_empty() {
            continue;
        }

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
    use serial_test::serial;
    use std::fs;
    use tempfile::TempDir;

    /// Helper function to run tests with a specific PATH value and ensure cleanup.
    ///
    /// # Arguments
    ///
    /// * `test_path` - Optional PATH value to use during the test. None means PATH will be unset.
    /// * `test` - The test function to run
    ///
    /// # Details
    ///
    /// This function will:
    /// 1. Save the current PATH
    /// 2. Set or unset PATH based on the test_path parameter
    /// 3. Run the test
    /// 4. Restore the original PATH, even if the test panics
    fn with_test_path<F>(test_path: Option<&str>, test: F)
    where
        F: FnOnce() + std::panic::UnwindSafe,
    {
        // Save original PATH
        let original_path = env::var("PATH").ok();

        // Set or remove test PATH
        match test_path {
            Some(path) => env::set_var("PATH", path),
            None => env::remove_var("PATH"),
        }

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

    /// Tests basic path validation with one valid and one invalid directory
    #[test]
    #[serial]
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

        with_test_path(Some(&test_path), || {
            let validation = validate_path().unwrap();
            assert_eq!(validation.existing_dirs.len(), 1);
            assert_eq!(validation.missing_dirs.len(), 1);
            assert!(validation.existing_dirs.contains(&valid_path));
            assert!(validation.missing_dirs.contains(&invalid_path));
        });
    }

    /// Tests validation with an empty PATH
    #[test]
    #[serial]
    fn test_validate_path_empty_path() {
        with_test_path(Some(""), || {
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

    /// Tests validation with an unset PATH
    #[test]
    #[serial]
    fn test_validate_unset_path() {
        with_test_path(None, || {
            // Check if PATH is unset or empty
            let path_var = env::var("PATH");
            println!("PATH after unset: {:?}", path_var);

            // Proceed if PATH is unset or empty
            if path_var.is_err() || path_var.as_ref().unwrap().trim().is_empty() {
                let validation = validate_path().unwrap();
                assert!(
                    validation.existing_dirs.is_empty(),
                    "Expected no existing directories with unset PATH, got: {:?}",
                    validation.existing_dirs
                );
                assert!(
                    validation.missing_dirs.is_empty(),
                    "Expected no missing directories with unset PATH, got: {:?}",
                    validation.missing_dirs
                );
            } else {
                panic!(
                    "Expected PATH to be unset or empty, but got: {:?}",
                    path_var.unwrap()
                );
            }
        });
    }

    /// Tests that directories are properly sorted in the output
    #[test]
    #[serial]
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

        with_test_path(Some(&test_path), || {
            let validation = validate_path().unwrap();
            assert_eq!(validation.missing_dirs.len(), 3);
            assert!(validation.missing_dirs[0] < validation.missing_dirs[1]);
            assert!(validation.missing_dirs[1] < validation.missing_dirs[2]);
        });
    }

    /// Tests validation with a mix of existing and missing directories
    #[test]
    #[serial]
    fn test_existing_and_missing_mix() {
        let temp_dir = TempDir::new().unwrap();
        let existing_a = temp_dir.path().join("existing_a");
        let existing_b = temp_dir.path().join("existing_b");
        let missing_a = temp_dir.path().join("missing_a");
        let missing_b = temp_dir.path().join("missing_b");

        fs::create_dir(&existing_a).unwrap();
        fs::create_dir(&existing_b).unwrap();

        // Use absolute paths for all directories
        let existing_a = existing_a.canonicalize().unwrap();
        let existing_b = existing_b.canonicalize().unwrap();
        let missing_a = missing_a.canonicalize().unwrap_or(missing_a.clone());
        let missing_b = missing_b.canonicalize().unwrap_or(missing_b.clone());

        // Use platform-specific path separator
        let _separator = std::path::MAIN_SEPARATOR.to_string();
        let path_separator = if cfg!(windows) { ";" } else { ":" };
        let test_path = format!(
            "{}{}{}{}{}{}{}",
            existing_b.to_string_lossy(),
            path_separator,
            missing_a.to_string_lossy(),
            path_separator,
            existing_a.to_string_lossy(),
            path_separator,
            missing_b.to_string_lossy()
        );

        with_test_path(Some(&test_path), || {
            let validation = validate_path().unwrap();

            // Debug output
            println!("Test PATH: {}", test_path);
            println!("Existing dirs: {:?}", validation.existing_dirs);
            println!("Missing dirs: {:?}", validation.missing_dirs);

            assert_eq!(
                validation.existing_dirs.len(),
                2,
                "Expected exactly 2 existing directories"
            );
            assert_eq!(
                validation.missing_dirs.len(),
                2,
                "Expected exactly 2 missing directories"
            );

            // Create sorted vectors for comparison
            let mut expected_existing = vec![existing_a.clone(), existing_b.clone()];
            expected_existing.sort();
            let mut expected_missing = vec![missing_a.clone(), missing_b.clone()];
            expected_missing.sort();

            let mut actual_existing = validation.existing_dirs.clone();
            actual_existing.sort();
            let mut actual_missing = validation.missing_dirs.clone();
            actual_missing.sort();

            assert_eq!(
                actual_existing, expected_existing,
                "Existing directories don't match expected"
            );
            assert_eq!(
                actual_missing, expected_missing,
                "Missing directories don't match expected"
            );
        });
    }
}
