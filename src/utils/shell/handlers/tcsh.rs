use super::ShellHandler;
use crate::utils::shell::types::{ModificationType, PathModification, ShellType};
use chrono::Local;
use dirs_next;
use regex::Regex;
use std::path::PathBuf;

pub struct TcshHandler {
    config_path: PathBuf,
}

impl TcshHandler {
    pub fn new() -> Self {
        let home_dir = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        Self {
            config_path: home_dir.join(".tcshrc"),
        }
    }
}

impl ShellHandler for TcshHandler {
    fn get_shell_type(&self) -> ShellType {
        ShellType::Tcsh
    }

    fn get_config_path(&self) -> PathBuf {
        self.config_path.clone()
    }

    fn parse_path_entries(&self, content: &str) -> Vec<PathBuf> {
        let mut entries = Vec::new();
        let setenv_regex = Regex::new(r"setenv\s+PATH\s+([^#\n]+)").unwrap();
        let set_regex = Regex::new(r"set\s+path\s*=\s*\((.*?)\)").unwrap();

        for line in content.lines() {
            let line = line.trim();

            // Handle setenv PATH ...
            if let Some(cap) = setenv_regex.captures(line) {
                if let Some(paths) = cap.get(1) {
                    for path in paths
                        .as_str()
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .split(':')
                    {
                        let expanded = shellexpand::tilde(path);
                        entries.push(PathBuf::from(expanded.to_string()));
                    }
                }
            }
            // Handle set path = (...)
            else if let Some(cap) = set_regex.captures(line) {
                if let Some(paths) = cap.get(1) {
                    for path in paths.as_str().split_whitespace() {
                        let expanded = shellexpand::tilde(path);
                        entries.push(PathBuf::from(expanded.to_string()));
                    }
                }
            }
        }

        entries
    }

    fn format_path_export(&self, entries: &[PathBuf]) -> String {
        let paths = entries
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>();

        format!(
            "\n# Updated by pathmaster on {}\nset path = ({})\nsetenv PATH {}\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            paths.join(" "),
            paths.join(":")
        )
    }

    fn detect_path_modifications(&self, content: &str) -> Vec<PathModification> {
        let mut modifications = Vec::new();
        let path_regex = Regex::new(r"(setenv\s+PATH|set\s+path\s*=)").unwrap();

        for (idx, line) in content.lines().enumerate() {
            if path_regex.is_match(line) {
                modifications.push(PathModification {
                    line_number: idx + 1,
                    content: line.to_string(),
                    modification_type: ModificationType::SetEnv,
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

#[cfg(test)]
mod tcsh_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_tcsh_path_parsing() {
        let handler = TcshHandler::new();
        let content = r#"
# Some config
setenv PATH /usr/bin:/usr/local/bin
set path = (/usr/bin /usr/local/bin ~/bin)
"#;

        let entries = handler.parse_path_entries(content);
        assert_eq!(entries.len(), 5); // 2 from setenv + 3 from set path
        assert!(entries.iter().any(|p| p.ends_with("usr/bin")));
        assert!(entries.iter().any(|p| p.ends_with("usr/local/bin")));
    }

    #[test]
    fn test_tcsh_path_formatting() {
        let handler = TcshHandler::new();
        let entries = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];

        let formatted = handler.format_path_export(&entries);
        assert!(formatted.contains("set path = ("));
        assert!(formatted.contains("setenv PATH"));
    }

    #[test]
    fn test_tcsh_config_update() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".tcshrc");

        let initial_content = r#"
# Initial config
set path = (/usr/bin /old/path)
setenv PATH /usr/bin:/old/path
"#;

        fs::write(&config_path, initial_content).unwrap();

        let mut handler = TcshHandler::new();
        handler.config_path = config_path.clone();

        let new_entries = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];

        handler.update_config(&new_entries).unwrap();

        let updated_content = fs::read_to_string(&config_path).unwrap();
        assert!(!updated_content.contains("/old/path"));
        assert!(updated_content.contains("/usr/bin"));
        assert!(updated_content.contains("/usr/local/bin"));
    }
}
