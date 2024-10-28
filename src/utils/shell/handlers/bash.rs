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

        let mut updated_content = content
            .lines()
            .enumerate()
            .filter(|(idx, _)| !modifications.iter().any(|m| m.line_number == idx + 1))
            .map(|(_, line)| line)
            .collect::<Vec<_>>()
            .join("\n");

        updated_content.push_str(&self.format_path_export(entries));

        updated_content
    }
}
