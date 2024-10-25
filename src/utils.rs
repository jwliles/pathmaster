//! Utility functions for PATH management
//!
//! This module provides core functionality for path manipulation,
//! validation, and shell configuration management.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

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
/// # use path_finder::utils;
/// let expanded = utils::expand_path("~/Documents");
/// ```
pub fn expand_path(path: &str) -> PathBuf {
    let expanded = shellexpand::tilde(path);
    PathBuf::from(expanded.to_string())
}

/// Validates whether a path entry is valid for inclusion in PATH.
///
/// # Criteria
/// - Path must exist in the filesystem
/// - Path must be a directory
///
/// # Arguments
/// * `path` - The path to validate
///
/// # Returns
/// * `true` if the path exists and is a directory
/// * `false` otherwise
///
/// # Example
/// ```rust
/// # use path_finder::utils;
/// # use std::path::Path;
/// let path = Path::new("/usr/bin");
/// let is_valid = utils::is_valid_path_entry(path);
/// ```
pub fn is_valid_path_entry(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

/// Updates shell configuration files with the new PATH.
///
/// # Process
/// 1. Identifies the appropriate shell configuration file
/// 2. Creates a backup of the existing configuration
/// 3. Removes existing PATH exports
/// 4. Adds new PATH export command
///
/// # Arguments
/// * `entries` - Vector of PathBuf containing the new PATH entries
///
/// # Returns
/// * `Ok(())` if the update was successful
/// * `Err(std::io::Error)` if any file operations failed
///
/// # Shell Support
/// - Bash (.bashrc)
/// - Zsh (.zshrc)
/// - Generic (.profile)
///
/// # Example
/// ```rust
/// # use path_finder::utils;
/// # use std::path::PathBuf;
/// let entries = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];
/// utils::update_shell_config(&entries).expect("Failed to update shell config");
/// ```
pub fn update_shell_config(entries: &[PathBuf]) -> std::io::Result<()> {
    let shell = env::var("SHELL").unwrap_or_default();
    let home_dir = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("/"));

    let config_file = if shell.contains("bash") {
        home_dir.join(".bashrc")
    } else if shell.contains("zsh") {
        home_dir.join(".zshrc")
    } else {
        home_dir.join(".profile")
    };

    // Create backup of existing config
    // Create backup of existing config
    let backup_path = config_file.with_extension("bak");
    if let Err(e) = fs::copy(&config_file, &backup_path) {
        eprintln!("Warning: Failed to create backup of shell config: {}", e);
        eprintln!("Backup would have been saved to: {}", backup_path.display());
    } else {
        println!(
            "Created backup of shell config at: {}",
            backup_path.display()
        );
    }

    // Read existing content
    let mut content = fs::read_to_string(&config_file)?;

    // Remove existing PATH exports
    content = content
        .lines()
        .filter(|line| !line.trim().starts_with("export PATH="))
        .collect::<Vec<&str>>()
        .join("\n");

    // Generate new PATH export command
    let new_path = env::join_paths(entries).unwrap();
    let export_command = format!(
        "\n# Updated PATH by pathfinder on {}\nexport PATH=\"{}\"\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        new_path.to_string_lossy()
    );

    // Write updated content
    std::fs::write(&config_file, content + &export_command)?;

    Ok(())
}

/// Gets the current PATH entries as a vector of PathBuf.
///
/// # Returns
/// * `Vec<PathBuf>` - Vector of current PATH entries
///
/// # Example
/// ```rust
/// # use path_finder::utils;
/// let current_paths = utils::get_path_entries();
/// ```
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
/// # use path_finder::utils;
/// # use std::path::PathBuf;
/// let new_paths = vec![PathBuf::from("/usr/bin")];
/// utils::set_path_entries(&new_paths);
/// ```
pub fn set_path_entries(entries: &[PathBuf]) {
    if let Ok(new_path) = env::join_paths(entries) {
        env::set_var("PATH", new_path);
    }
}
