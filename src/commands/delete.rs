//! Command implementation for removing directories from PATH.
//!
//! This module handles:
//! - Removing specified directories from PATH
//! - Creating backups before modification
//! - Updating shell configuration
//! - Maintaining PATH integrity

use crate::backup;
use crate::utils;

/// Executes the delete command to remove directories from PATH
///
/// # Arguments
///
/// * `directories` - A slice of strings containing directories to remove
///
/// # Example
///
/// ```
/// let dirs = vec![String::from("~/old/bin")];
/// commands::delete::execute(&dirs);
/// ```
pub fn execute(directories: &[String]) {
    // Backup current PATH
    if let Err(e) = backup::create_backup() {
        eprintln!("Error creating backup: {}", e);
        return;
    }

    // Get current PATH
    let mut path_entries = utils::get_path_entries();

    // Remove the directories
    let original_len = path_entries.len();
    for directory in directories {
        let dir_path = utils::expand_path(directory);
        path_entries.retain(|p| p != &dir_path);
    }

    if path_entries.len() == original_len {
        println!("None of the directories were found in PATH.");
        return;
    }

    // Update PATH
    utils::set_path_entries(&path_entries);

    // Make persistent changes (update shell config)
    if let Err(e) = utils::update_shell_config(&path_entries) {
        eprintln!("Error updating shell configuration: {}", e);
        return;
    }

    println!("Successfully removed directories from PATH.");
}
