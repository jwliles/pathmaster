//! Command implementation for removing invalid paths from PATH.
//!
//! This module handles:
//! - Identifying non-existent directories in PATH
//! - Creating backups before modification
//! - Removing invalid entries
//! - Updating shell configuration

use crate::backup;
use crate::utils;

/// Executes the flush command to remove non-existing paths from PATH
///
/// This function will:
/// 1. Create a backup of the current PATH
/// 2. Check each directory in PATH for existence
/// 3. Remove directories that don't exist
/// 4. Update the PATH environment variable
/// 5. Update shell configuration
///
/// # Example
///
/// ```
/// commands::flush::execute();
/// // This will remove all non-existing directories from PATH
/// ```
pub fn execute() {
    // Backup current PATH
    if let Err(e) = backup::create_backup() {
        eprintln!("Error creating backup: {}", e);
        return;
    }

    // Get current PATH entries
    let mut path_entries = utils::get_path_entries();

    // Identify non-existing paths
    let original_len = path_entries.len();
    path_entries.retain(|p| p.exists());

    let removed_count = original_len - path_entries.len();

    if removed_count == 0 {
        println!("No invalid paths were found in your PATH.");
        return;
    }

    // Update PATH
    utils::set_path_entries(&path_entries);

    // Update shell configuration
    if let Err(e) = utils::update_shell_config(&path_entries) {
        eprintln!("Error updating shell configuration: {}", e);
        return;
    }

    println!("Removed {} invalid path(s) from your PATH.", removed_count);
}
