// src/backup/show.rs

use super::core::get_backup_dir;
use std::fs;

/// Displays the history of PATH backups
///
/// Lists all available backups in chronological order
pub fn show_history() {
    let backup_dir = match get_backup_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting backup directory: {}", e);
            return;
        }
    };

    match fs::read_dir(&backup_dir) {
        Ok(entries) => {
            println!("Available backups:");
            for entry in entries.flatten() {
                println!("- {}", entry.file_name().to_string_lossy());
            }
        }
        Err(_) => {
            println!("No backups found.");
        }
    }
}
