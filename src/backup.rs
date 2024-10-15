use chrono::Local;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Backup {
    timestamp: String,
    path: String,
}

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

pub fn get_backup_dir() -> PathBuf {
    let home_dir = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("/"));
    home_dir.join(".pathfinder_backups")
}
