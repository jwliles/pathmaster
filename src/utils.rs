//! Utility functions for PATH manipulation and shell configuration management.
//!
//! This module provides core functionality for:
//! - Path expansion (e.g., expanding ~ to home directory)
//! - PATH environment variable manipulation
//! - Shell configuration file management

use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

/// Expands a path string, replacing ~ with the user's home directory
///
/// # Arguments
///
/// * `path` - A string slice containing the path to expand
///
/// # Returns
///
/// Returns a PathBuf with the expanded path
///
/// # Example
///
/// ```
/// let expanded = utils::expand_path("~/bin");
/// assert!(expanded.to_string_lossy().contains("/home/"));
/// ```
pub fn expand_path(path: &str) -> PathBuf {
    let expanded = shellexpand::tilde(path);
    PathBuf::from(expanded.to_string())
}

/// Retrieves current PATH entries as a vector of PathBuf
///
/// # Returns
///
/// Returns a Vec<PathBuf> containing all current PATH entries
///
/// # Example
///
/// ```
/// let entries = utils::get_path_entries();
/// println!("Current PATH has {} entries", entries.len());
/// ```
pub fn get_path_entries() -> Vec<PathBuf> {
    env::var_os("PATH")
        .map(|paths| env::split_paths(&paths).collect())
        .unwrap_or_default()
}

/// Updates the PATH environment variable with new entries
///
/// # Arguments
///
/// * `entries` - A slice of PathBuf containing the new PATH entries
///
/// # Example
///
/// ```
/// let mut entries = utils::get_path_entries();
/// entries.push(PathBuf::from("/usr/local/bin"));
/// utils::set_path_entries(&entries);
/// ```
pub fn set_path_entries(entries: &[PathBuf]) {
    let new_path = env::join_paths(entries).expect("Failed to join PATH entries");
    env::set_var("PATH", &new_path);
}

/// Updates shell configuration file with new PATH entries
///
/// # Arguments
///
/// * `entries` - A slice of PathBuf containing the PATH entries to save
///
/// # Returns
///
/// Returns std::io::Result indicating success or failure
///
/// # Example
///
/// ```
/// let entries = utils::get_path_entries();
/// utils::update_shell_config(&entries)?;
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

    let new_path = env::join_paths(entries).unwrap();
    let export_command = format!(
        "\n# Updated PATH by pathmaster\nexport PATH=\"{}\"\n",
        new_path.to_string_lossy()
    );

    let mut file = OpenOptions::new().append(true).open(config_file)?;
    file.write_all(export_command.as_bytes())?;

    Ok(())
}
