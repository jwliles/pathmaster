//! Core backup functionality for pathmaster.

use chrono::Local;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::io;
use std::path::PathBuf;
use std::sync::Mutex;

lazy_static! {
    static ref BACKUP_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);
}

/// Represents a PATH backup with timestamp and path data
#[derive(Debug, Serialize, Deserialize)]
pub struct Backup {
    /// Timestamp when backup was created
    pub timestamp: String,
    /// Complete PATH string at backup time
    pub path: String,
}

/// Sets a custom backup directory (primarily for testing)
#[allow(dead_code)]
pub fn set_backup_dir(dir: PathBuf) -> io::Result<()> {
    let mut backup_dir = BACKUP_DIR.lock().map_err(|_| {
        io::Error::new(
            io::ErrorKind::Other,
            "Failed to lock backup directory mutex",
        )
    })?;
    *backup_dir = Some(dir);
    Ok(())
}

/// Gets the directory where backups are stored
///
/// # Returns
/// * `PathBuf` containing the path to the backup directory
pub fn get_backup_dir() -> io::Result<PathBuf> {
    let backup_dir = BACKUP_DIR.lock().map_err(|_| {
        io::Error::new(
            io::ErrorKind::Other,
            "Failed to lock backup directory mutex",
        )
    })?;

    Ok(backup_dir.clone().unwrap_or_else(|| {
        let home_dir = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        home_dir.join(".pathmaster/backups")
    }))
}

/// Creates a new backup of the current PATH environment
///
/// # Returns
/// * `Ok(())` on successful backup creation
/// * `Err(io::Error)` if backup creation fails
pub fn create_backup() -> io::Result<()> {
    let backup_dir = get_backup_dir()?;

    // Create backup directory if it doesn't exist
    fs::create_dir_all(&backup_dir)?;

    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let path = env::var("PATH").unwrap_or_default();

    let backup = Backup {
        timestamp: timestamp.clone(),
        path,
    };

    let backup_file = backup_dir.join(format!("backup_{}.json", timestamp));
    println!("Creating backup at: {:?}", backup_file); // Debug print

    let file = File::create(&backup_file)?;
    serde_json::to_writer_pretty(file, &backup)?;

    // Verify file was created
    if !backup_file.exists() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create backup file at {:?}", backup_file),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::fs;
    use tempfile::TempDir;

    /// Helper function to count backup files in a directory
    fn count_backup_files(dir: &PathBuf) -> io::Result<usize> {
        let count = fs::read_dir(dir)?
            .filter(|entry| {
                entry
                    .as_ref()
                    .ok()
                    .and_then(|e| {
                        let path = e.path();
                        path.extension().map(|ext| ext == "json")
                    })
                    .unwrap_or(false)
            })
            .count();
        println!("Found {} backup files in {:?}", count, dir); // Debug print
        Ok(count)
    }

    #[test]
    #[serial]
    fn test_backup_creation() -> io::Result<()> {
        // Create temporary directory
        let temp_dir = TempDir::new()?;
        let backup_dir = temp_dir.path().to_path_buf();
        println!("Test backup directory: {:?}", backup_dir);

        // Set test backup directory
        set_backup_dir(backup_dir.clone())?;

        // Verify the backup directory is set correctly
        assert_eq!(
            get_backup_dir()?,
            backup_dir,
            "Backup directory not set correctly"
        );

        // Set test PATH
        let test_path = "/usr/bin:/usr/local/bin".to_string();
        env::set_var("PATH", &test_path);

        // Verify initial state
        assert_eq!(
            count_backup_files(&backup_dir)?,
            0,
            "Expected no backup files initially"
        );

        // Create backup
        create_backup()?;

        // List directory contents for debugging
        println!("Directory contents after backup:");
        for entry in fs::read_dir(&backup_dir)? {
            if let Ok(entry) = entry {
                println!("  {:?}", entry.path());
            }
        }

        // Verify backup was created
        let backup_count = count_backup_files(&backup_dir)?;
        assert_eq!(
            backup_count,
            1,
            "Expected 1 backup file, found {}. Directory contents: {:?}",
            backup_count,
            fs::read_dir(&backup_dir)?.collect::<Vec<_>>()
        );

        // Find and verify the backup file
        let backup_files: Vec<_> = fs::read_dir(&backup_dir)?
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "json"))
            .collect();

        assert_eq!(backup_files.len(), 1, "Expected exactly one backup file");

        let backup_content = fs::read_to_string(backup_files[0].path())?;
        let backup: Backup = serde_json::from_str(&backup_content)?;

        assert_eq!(
            backup.path, test_path,
            "Backup PATH does not match test PATH"
        );

        Ok(())
    }

    #[test]
    #[serial]
    fn test_multiple_backups() -> io::Result<()> {
        // Create temporary directory
        let temp_dir = TempDir::new()?;
        let backup_dir = temp_dir.path().to_path_buf();
        println!("Test backup directory: {:?}", backup_dir);

        // Set test backup directory
        set_backup_dir(backup_dir.clone())?;

        // Verify the backup directory is set correctly
        assert_eq!(
            get_backup_dir()?,
            backup_dir,
            "Backup directory not set correctly"
        );

        // Create multiple backups
        create_backup()?;
        std::thread::sleep(std::time::Duration::from_secs(1)); // Ensure unique timestamps
        create_backup()?;

        // List directory contents for debugging
        println!("Directory contents after backups:");
        for entry in fs::read_dir(&backup_dir)? {
            if let Ok(entry) = entry {
                println!("  {:?}", entry.path());
            }
        }

        let count = count_backup_files(&backup_dir)?;
        assert_eq!(count, 2, "Expected 2 backup files, found {}", count);

        Ok(())
    }

    #[test]
    #[serial]
    fn test_backup_dir_creation() -> io::Result<()> {
        // Create temporary directory
        let temp_dir = TempDir::new()?;
        let backup_dir = temp_dir.path().join("new_backups");
        println!("Test backup directory: {:?}", backup_dir);

        // Set test backup directory
        set_backup_dir(backup_dir.clone())?;

        // Verify the backup directory is set correctly
        assert_eq!(
            get_backup_dir()?,
            backup_dir,
            "Backup directory not set correctly"
        );

        assert!(
            !backup_dir.exists(),
            "Backup directory should not exist initially"
        );

        create_backup()?;

        assert!(
            backup_dir.exists(),
            "Backup directory should be created after backup"
        );
        assert!(
            backup_dir.is_dir(),
            "Backup directory path should be a directory"
        );

        Ok(())
    }
}
