//! PATH backup management functionality.
//!
//! This module provides functionality for:
//! - Creating timestamped backups of PATH
//! - Listing backup history
//! - Managing backup storage location

use chrono::Local;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::path::PathBuf;

/// Represents a PATH backup with timestamp
#[derive(Serialize, Deserialize)]
struct Backup {
    /// Timestamp when backup was created
    timestamp: String,
    /// Complete PATH string at backup time
    path: String,
}

/// Creates a new backup of the current PATH
///
/// # Returns
///
/// Returns std::io::Result indicating success or failure
///
/// # Example
///
/// ```
/// match backup::create_backup() {
///     Ok(_) => println!("Backup created successfully"),
///     Err(e) => eprintln!("Failed to create backup: {}", e),
/// }
/// ```
pub fn create_backup() -> std::io::Result<()> {
    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let path = env::var("PATH").unwrap_or_default();

    let backup = Backup {
        timestamp: timestamp.clone(),
        path,
    };

    let backup_dir = get_backup_dir();
    fs::create_dir_all(&backup_dir)?;

    let backup_file = backup_dir.join(format!("backup_{}.json", timestamp));
    let file = File::create(backup_file)?;
    serde_json::to_writer_pretty(file, &backup)?;

    Ok(())
}

/// Displays the history of PATH backups
///
/// Lists all available backups in chronological order
pub fn show_history() {
    let backup_dir = get_backup_dir();

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

/// Gets the directory where backups are stored
///
/// # Returns
///
/// Returns a PathBuf pointing to the backup directory
pub fn get_backup_dir() -> PathBuf {
    let home_dir = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("/"));
    home_dir.join(".pathfinder_backups")
}
