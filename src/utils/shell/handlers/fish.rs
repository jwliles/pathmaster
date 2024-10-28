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

        // Remove existing PATH modifications
        let mut updated_content = content
            .lines()
            .enumerate()
            .filter(|(idx, _)| !modifications.iter().any(|m| m.line_number == idx + 1))
            .map(|(_, line)| line)
            .collect::<Vec<_>>()
            .join("\n");

        // Add new PATH configuration
        updated_content.push_str(&self.format_path_export(entries));

        updated_content
    }
}
