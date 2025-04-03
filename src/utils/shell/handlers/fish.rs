use super::ShellHandler;
use crate::utils::shell::types::{ModificationType, PathModification, ShellType};
use chrono::Local;
use dirs_next;
use regex::Regex;
use std::path::PathBuf;

pub struct FishHandler {
    config_path: PathBuf,
}

impl FishHandler {
    pub fn new() -> Self {
        let home_dir = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        Self {
            config_path: home_dir.join(".config/fish/config.fish"),
        }
    }
}

impl ShellHandler for FishHandler {
    fn get_shell_type(&self) -> ShellType {
        ShellType::Fish
    }

    fn get_config_path(&self) -> PathBuf {
        self.config_path.clone()
    }

    fn parse_path_entries(&self, content: &str) -> Vec<PathBuf> {
        let mut entries = Vec::new();
        let path_regex = Regex::new(r"fish_add_path\s+(.+)$").unwrap();

        for line in content.lines() {
            if let Some(cap) = path_regex.captures(line.trim()) {
                if let Some(path) = cap.get(1) {
                    let expanded = shellexpand::tilde(path.as_str());
                    entries.push(PathBuf::from(expanded.to_string()));
                }
            }
        }

        entries
    }

    fn format_path_export(&self, entries: &[PathBuf]) -> String {
        let mut output = String::new();
        output.push_str("\n# Updated by pathmaster on ");
        output.push_str(&Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        output.push_str("\n");

        // Clear existing PATH
        output.push_str("set -e PATH\n");

        // Add each path using fish_add_path
        for entry in entries {
            output.push_str(&format!("fish_add_path {}\n", entry.display()));
        }

        output
    }

    fn detect_path_modifications(&self, content: &str) -> Vec<PathModification> {
        let mut modifications = Vec::new();
        let path_regex = Regex::new(r"(fish_add_path|set -gx PATH)").unwrap();

        for (idx, line) in content.lines().enumerate() {
            if path_regex.is_match(line) {
                modifications.push(PathModification {
                    line_number: idx + 1,
                    content: line.to_string(),
                    modification_type: ModificationType::FishPath,
                });
            }
        }

        modifications
    }

    fn update_path_in_config(&self, content: &str, entries: &[PathBuf]) -> String {
        let modifications = self.detect_path_modifications(content);
        let new_path_config = self.format_path_export(entries);
        
        // If we found existing PATH modifications, update in place
        if !modifications.is_empty() {
            // Sort by line number in descending order to avoid index shifting
            let mut sorted_mods = modifications.clone();
            sorted_mods.sort_by(|a, b| b.line_number.cmp(&a.line_number));
            
            // First modification is where we'll insert our new config
            let first_mod = sorted_mods.last().unwrap().line_number - 1;
            
            // Convert to lines for manipulation
            let mut lines: Vec<&str> = content.lines().collect();
            
            // Remove all existing PATH declarations
            for modification in sorted_mods {
                lines.remove(modification.line_number - 1);
            }
            
            // Insert new config at the position of the first PATH declaration
            // Remove newline prefix if it exists
            let new_config = new_path_config.trim_start_matches('\n');
            for line in new_config.lines().rev() {
                lines.insert(first_mod, line);
            }
            
            return lines.join("\n");
        } else {
            // No existing PATH declarations found, append to end
            return content.to_string() + &new_path_config;
        }
    }
}
