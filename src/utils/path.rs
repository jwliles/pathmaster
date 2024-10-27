//! Utility functions for PATH management.
//!
//! This module provides core functionality for:
//! - Path manipulation and expansion
//! - Path validation
//! - PATH environment variable management
//!
//! For shell configuration management, see the `shell` module.

use std::env;
use std::path::PathBuf;

/// Expands a path string, resolving home directory (~) and environment variables.
///
/// # Arguments
/// * `path` - The path string to expand
///
/// # Returns
/// * `PathBuf` - The expanded path
///
/// # Example
/// ```rust
/// # use pathmaster::utils;
/// let expanded = utils::expand_path("~/Documents");
/// assert!(expanded.to_string_lossy().contains("Documents"));
/// ```
/// Expands a path string, resolving home directory (~) and environment variables.
pub fn expand_path(path: &str) -> PathBuf {
    let expanded = shellexpand::tilde(path);
    PathBuf::from(expanded.to_string())
}

/// Gets the current PATH entries as a vector of PathBuf.
///
/// # Returns
/// * `Vec<PathBuf>` - Vector of current PATH entries
///
/// # Example
/// ```rust
/// # use pathmaster::utils;
/// let current_paths = utils::get_path_entries();
/// ```
/// Gets the current PATH entries as a vector of PathBuf.
pub fn get_path_entries() -> Vec<PathBuf> {
    env::var_os("PATH")
        .map(|paths| env::split_paths(&paths).collect())
        .unwrap_or_default()
}

/// Sets the PATH environment variable to the provided entries.
///
/// # Arguments
/// * `entries` - Vector of PathBuf to set as new PATH
///
/// # Example
/// ```rust
/// # use pathmaster::utils;
/// # use std::path::PathBuf;
/// let new_paths = vec![PathBuf::from("/usr/bin")];
/// utils::set_path_entries(&new_paths);
/// ```
/// Sets the PATH environment variable to the provided entries.
pub fn set_path_entries(entries: &[PathBuf]) {
    if let Ok(new_path) = env::join_paths(entries) {
        env::set_var("PATH", new_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::validator::is_valid_path_entry;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_expand_path() {
        let home = dirs_next::home_dir().unwrap();
        let expanded = expand_path("~/test");
        assert_eq!(expanded, home.join("test"));
    }

    #[test]
    fn test_is_valid_path_entry() {
        let temp_dir = TempDir::new().unwrap();
        assert!(is_valid_path_entry(temp_dir.path()));

        let non_existent = temp_dir.path().join("non_existent");
        assert!(!is_valid_path_entry(&non_existent));
    }

    #[test]
    fn test_get_set_path_entries() {
        // Save original PATH
        let original_path = env::var("PATH").ok();

        // Test paths
        let test_paths = vec![PathBuf::from("/test/path1"), PathBuf::from("/test/path2")];

        // Set new PATH
        set_path_entries(&test_paths);

        // Get and verify PATH
        let current_paths = get_path_entries();
        assert_eq!(current_paths, test_paths);

        // Restore original PATH
        if let Some(path) = original_path {
            env::set_var("PATH", path);
        }
    }
}
