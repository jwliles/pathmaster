use super::ShellHandler;
use crate::utils::shell::types::{ModificationType, PathModification, ShellType};
use chrono::Local;
use dirs_next;
use regex::Regex;
use std::path::PathBuf;

pub struct BashHandler {
    config_path: PathBuf,
}

impl BashHandler {
    pub fn new() -> Self {
        let home_dir = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        Self {
            config_path: home_dir.join(".bashrc"),
        }
    }

    fn parse_path_additions(&self, line: &str) -> Option<PathBuf> {
        let addition_regex = Regex::new(r"PATH=.*:([^:]+)\s*$").unwrap();
        if let Some(cap) = addition_regex.captures(line) {
            if let Some(path) = cap.get(1) {
                let expanded = shellexpand::tilde(path.as_str());
                return Some(PathBuf::from(expanded.to_string()));
            }
        }
        None
    }
}

impl ShellHandler for BashHandler {
    fn get_shell_type(&self) -> ShellType {
        ShellType::Bash
    }

    fn get_config_path(&self) -> PathBuf {
        self.config_path.clone()
    }

    fn parse_path_entries(&self, content: &str) -> Vec<PathBuf> {
        let mut entries = Vec::new();
        let export_regex = Regex::new(r#"export\s+PATH=["']?([^"']+)["']?"#).unwrap();

        for line in content.lines() {
            let line = line.trim();

            // Handle export PATH=...
            if let Some(cap) = export_regex.captures(line) {
                if let Some(paths) = cap.get(1) {
                    for path in paths.as_str().split(':') {
                        let expanded = shellexpand::tilde(path);
                        entries.push(PathBuf::from(expanded.to_string()));
                    }
                }
            }
            // Handle PATH additions
            else if line.contains("PATH=$PATH:") || line.contains("PATH=\"$PATH:") {
                if let Some(path) = self.parse_path_additions(line) {
                    entries.push(path);
                }
            }
        }

        entries
    }

    fn format_path_export(&self, entries: &[PathBuf]) -> String {
        let paths = entries
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(":");

        format!(
            "\n# Updated by pathmaster on {}\nexport PATH=\"{}\"\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            paths
        )
    }

    fn detect_path_modifications(&self, content: &str) -> Vec<PathModification> {
        let mut modifications = Vec::new();
        let path_regex = Regex::new(r"(export\s+PATH=|PATH=\$PATH:)").unwrap();

        for (idx, line) in content.lines().enumerate() {
            if path_regex.is_match(line) {
                let mod_type = if line.contains("PATH=$PATH:") {
                    ModificationType::Addition
                } else {
                    ModificationType::Assignment
                };

                modifications.push(PathModification {
                    line_number: idx + 1,
                    content: line.to_string(),
                    modification_type: mod_type,
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
