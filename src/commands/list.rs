//! Command implementation for listing PATH entries.
//!
//! This module provides functionality to:
//! - Display all current PATH entries
//! - Format output for readability
//! - Show full paths with proper display formatting

use crate::utils;

/// Executes the list command to display current PATH entries
///
/// Lists all directories currently in PATH, with each entry on a new line
/// prefixed with a bullet point for better readability.
///
/// # Example
///
/// ```
/// commands::list::execute();
/// // Output example:
/// // Current PATH entries:
/// // - /usr/local/bin
/// // - /usr/bin
/// // - ~/custom/bin
/// ```
pub fn execute() {
    let path_entries = utils::get_path_entries();

    println!("Current PATH entries:");
    for path in path_entries {
        println!("- {}", path.display());
    }
}
