//! Path management functionality for removing invalid entries from PATH.
//!
//! This module provides functionality to:
//! - Identify and remove invalid PATH entries
//! - Update shell configuration files
//! - Maintain backups of configurations
//! - Provide detailed feedback about changes

use crate::backup;
use crate::commands::validator::is_valid_path_entry;
use crate::utils;
use std::path::PathBuf;

/// Removes invalid directories from the PATH environment variable.
pub fn execute() {
    // Backup current PATH
    if let Err(e) = backup::create_backup() {
        eprintln!("Error creating backup: {}", e);
        return;
    }

    // Get current PATH entries
    let current_entries = utils::get_path_entries();
    let original_count = current_entries.len();

    // Filter out non-existing paths
    let valid_entries: Vec<PathBuf> = current_entries
        .into_iter()
        .filter(|path| {
            if is_valid_path_entry(path) {
                true
            } else {
                println!("Removing invalid path: {}", path.display());
                false
            }
        })
        .collect();

    let removed_count = original_count - valid_entries.len();

    if removed_count == 0 {
        println!("No invalid paths found in PATH.");
        return;
    }

    // Update PATH environment variable
    utils::set_path_entries(&valid_entries);

    // Update shell configuration files
    match utils::update_shell_config(&valid_entries) {
        Ok(_) => {
            println!(
                "Successfully removed {} invalid path(s) and updated shell configuration.",
                removed_count
            );
        }
        Err(e) => {
            eprintln!("Error updating shell configuration: {}", e);
            println!("Warning: PATH environment variable was updated for current session only.");
            println!("To make changes permanent, you'll need to manually update your shell configuration.");
        }
    }
}
