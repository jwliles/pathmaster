use crate::backup;
use crate::utils;
use std::path::PathBuf;

pub fn execute(directories: &[String]) {
    // Expand and normalize the directory paths
    let dirs_to_add: Vec<PathBuf> = directories
        .iter()
        .map(|dir| utils::expand_path(dir))
        .collect();

    // Backup current PATH
    if let Err(e) = backup::create_backup() {
        eprintln!("Error creating backup: {}", e);
        return;
    }

    // Get current PATH
    let mut path_entries = utils::get_path_entries();

    // Track the number of directories added
    let mut added_count = 0;

    for dir_path in dirs_to_add {
        if !dir_path.is_dir() {
            eprintln!(
                "Warning: '{}' is not a valid directory.",
                dir_path.display()
            );
            continue;
        }

        if path_entries.contains(&dir_path) {
            println!("Directory '{}' is already in PATH.", dir_path.display());
            continue;
        }

        // Add the new directory
        path_entries.push(dir_path.clone());
        added_count += 1;
        println!("Added '{}' to PATH.", dir_path.display());
    }

    if added_count > 0 {
        // Update PATH
        utils::set_path_entries(&path_entries);

        // Update shell configuration
        if let Err(e) = utils::update_shell_config(&path_entries) {
            eprintln!("Error updating shell configuration: {}", e);
            return;
        }

        println!("Successfully added {} directory(ies) to PATH.", added_count);
    } else {
        println!("No new directories were added to PATH.");
    }
}
