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
            "export PATH=\"{}\" # Updated by pathmaster on {}",
            paths,
            Local::now().format("%Y-%m-%d %H:%M:%S")
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
            // Get all lines
            let mut lines: Vec<&str> = content.lines().collect();
            
            // Find the first path modification (which is where we'll update)
            let mut sorted_mods = modifications.clone();
            sorted_mods.sort_by(|a, b| a.line_number.cmp(&b.line_number));
            let first_mod = sorted_mods.first().unwrap().line_number - 1;
            
            // Replace only the first path declaration
            lines[first_mod] = &new_path_config;
            
            // If there are more path declarations, comment them out rather than removing
            // Removing could cause issues with line numbers in subsequent updates
            for &PathModification{line_number, ..} in sorted_mods.iter().skip(1) {
                let index = line_number - 1;
                if index < lines.len() {
                    lines[index] = &format!("# DISABLED by pathmaster: {}", lines[index]);
                }
            }
            
            return lines.join("\n");
        } else {
            // No existing PATH declarations found, append to end
            if content.ends_with('\n') {
                return format!("{}{}", content, new_path_config);
            } else {
                return format!("{}\n{}", content, new_path_config);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_bash_path_parsing() {
        let handler = BashHandler::new();
        let content = r#"
# Some config
export PATH="/usr/bin:/usr/local/bin"
PATH=$PATH:~/bin
"#;

        let entries = handler.parse_path_entries(content);
        assert_eq!(entries.len(), 3);
        assert!(entries.iter().any(|p| p.ends_with("usr/bin")));
        assert!(entries.iter().any(|p| p.ends_with("usr/local/bin")));
    }

    #[test]
    fn test_bash_path_formatting() {
        let handler = BashHandler::new();
        let entries = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];

        let formatted = handler.format_path_export(&entries);
        assert!(formatted.contains("export PATH=\""));
        assert!(formatted.contains("/usr/bin:/usr/local/bin"));
    }

    #[test]
    fn test_bash_config_update() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".bashrc");

        let initial_content = r#"
# Initial config
export PATH="/usr/bin:/old/path"
PATH=$PATH:/another/old/path
"#;

        fs::write(&config_path, initial_content).unwrap();

        let mut handler = BashHandler::new();
        handler.config_path = config_path.clone();

        let new_entries = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];

        handler.update_config(&new_entries).unwrap();

        let updated_content = fs::read_to_string(&config_path).unwrap();
        assert!(!updated_content.contains("/old/path"));
        assert!(updated_content.contains("/usr/bin"));
        assert!(updated_content.contains("/usr/local/bin"));
    }
    
    #[test]
    fn test_bash_in_place_update() {
        let handler = BashHandler::new();
        
        let content = r#"
# Header comment
# Some other configuration
export EDITOR=vim

# PATH configuration
export PATH="/usr/bin:/old/path"

# More configuration below
alias ls='ls --color=auto'
"#;

        let new_entries = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];
        let updated_content = handler.update_path_in_config(content, &new_entries);
        
        // Verify the PATH was updated in-place
        let lines: Vec<&str> = updated_content.lines().collect();
        
        // Find where the PATH declaration is in the updated content
        let mut path_line_index = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.contains("export PATH=") && !line.contains("DISABLED") {
                path_line_index = i;
                break;
            }
        }
        
        // Check that PATH is still at the same line (line 9)
        assert_eq!(path_line_index, 9, "PATH should remain at the same position");
        
        // Check that PATH is still between the EDITOR and alias lines
        let editor_line_index = lines.iter().position(|&line| line.contains("export EDITOR=")).unwrap();
        let alias_line_index = lines.iter().position(|&line| line.contains("alias ls=")).unwrap();
        
        assert!(editor_line_index < path_line_index, "PATH should be after EDITOR line");
        assert!(path_line_index < alias_line_index, "PATH should be before alias line");
        
        // Check content
        assert!(!updated_content.contains("/old/path") || updated_content.contains("DISABLED"));
        assert!(updated_content.contains("/usr/bin"));
        assert!(updated_content.contains("/usr/local/bin"));
        assert!(updated_content.contains("# Updated by pathmaster on"));
    }
}
