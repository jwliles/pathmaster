//! Command implementation for restoring PATH from backups.
//!
//! This module handles:
//! - Restoring PATH from specified backup files
//! - Finding and using the most recent backup
//! - Validating backup files
//! - Updating shell configuration after restore

use crate::backup::core::get_backup_dir;
use crate::utils;
use std::env;
use std::fs::File;
use std::io::Read;

/// Executes the restore command to recover PATH from a backup
///
/// # Arguments
///
/// * `timestamp` - Optional timestamp string to specify which backup to restore.
///                 If None, restores from the most recent backup.
///
/// # Example
///
/// ```
/// // Restore from specific backup
/// let timestamp = Some(String::from("20240321120000"));
/// commands::restore::execute(&timestamp);
///
/// // Restore from most recent backup
/// commands::restore::execute(&None);
/// ```
pub fn execute(timestamp: &Option<String>) {
    let backup_dir = match get_backup_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting backup directory: {}", e);
            return;
        }
    };

    let backup_file = match timestamp {
        Some(ts) => backup_dir.join(format!("backup_{}.json", ts)),
        None => {
            // Get the most recent backup
            match get_latest_backup(&backup_dir) {
                Some(file) => file,
                None => {
                    println!("No backups found.");
                    return;
                }
            }
        }
    };

    if !backup_file.exists() {
        println!("Backup file not found: {}", backup_file.display());
        return;
    }

    // Read the backup file
    let mut file = File::open(&backup_file).expect("Failed to open backup file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read backup file");

    // Deserialize the backup
    let backup: serde_json::Value =
        serde_json::from_str(&contents).expect("Failed to parse backup file");
    let path = backup["path"].as_str().unwrap_or_default();

    // Update PATH
    env::set_var("PATH", path);

    // Update shell configuration
    if let Err(e) = utils::update_shell_config(&utils::get_path_entries()) {
        eprintln!("Error updating shell configuration: {}", e);
        return;
    }

    println!("PATH restored from backup: {}", backup_file.display());
}

/// Gets the most recent backup file
///
/// # Arguments
///
/// * `backup_dir` - PathBuf pointing to the backup directory
///
/// # Returns
///
/// Option containing PathBuf to the most recent backup file,
/// or None if no backups exist
pub fn get_latest_backup(backup_dir: &std::path::Path) -> Option<std::path::PathBuf> {
    let mut backups: Vec<_> = std::fs::read_dir(backup_dir).ok()?.flatten().collect();
    backups.sort_by_key(|dir| dir.file_name());
    backups.last().map(|entry| entry.path())
}
