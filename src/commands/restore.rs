use crate::backup::get_backup_dir;
use crate::utils;
use std::env;
use std::fs::File;
use std::io::Read;

pub fn execute(timestamp: &Option<String>) {
    let backup_dir = get_backup_dir();

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

fn get_latest_backup(backup_dir: &std::path::Path) -> Option<std::path::PathBuf> {
    let mut backups: Vec<_> = std::fs::read_dir(backup_dir).ok()?.flatten().collect();
    backups.sort_by_key(|dir| dir.file_name());
    backups.last().map(|entry| entry.path())
}
