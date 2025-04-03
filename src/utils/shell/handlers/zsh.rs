use super::ShellHandler;
use crate::utils::shell::types::{ModificationType, PathModification, ShellType};
use chrono::Local;
use regex::Regex;
use std::path::PathBuf;

pub struct ZshHandler {
    config_path: PathBuf,
}

impl ZshHandler {
    pub fn new() -> Self {
        let home_dir = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        Self {
            config_path: home_dir.join(".zshrc"),
        }
    }

    fn find_path_arrays(&self, content: &str) -> Vec<PathModification> {
        let mut modifications = Vec::new();
        // Look for various patterns related to path configuration
        
        // Regex for path=(...) pattern
        let path_array_regex = Regex::new(r"path=\(.*?\)").unwrap();
        
        // Regex for path+=(...) pattern
        let path_append_regex = Regex::new(r"path\+=\(").unwrap();
        
        // Search line by line to get accurate line numbers
        for (line_idx, line) in content.lines().enumerate() {
            if path_array_regex.is_match(line) {
                modifications.push(PathModification {
                    line_number: line_idx + 1, // Line numbers are 1-based
                    content: line.to_string(),
                    modification_type: ModificationType::ArrayModification,
                });
            } else if path_append_regex.is_match(line) {
                // This handles multi-line path+=(...) constructs
                modifications.push(PathModification {
                    line_number: line_idx + 1,
                    content: line.to_string(),
                    modification_type: ModificationType::ArrayModification,
                });
            }
        }
        
        modifications
    }
}

impl ShellHandler for ZshHandler {
    fn get_shell_type(&self) -> ShellType {
        ShellType::Zsh
    }

    fn get_config_path(&self) -> PathBuf {
        self.config_path.clone()
    }

    fn parse_path_entries(&self, content: &str) -> Vec<PathBuf> {
        let mut entries = Vec::new();

        // Handle single-line path array: path=(...)
        if let Some(path_array) = content
            .lines()
            .find(|line| line.trim().starts_with("path=("))
        {
            let paths = path_array
                .trim()
                .trim_start_matches("path=(")
                .trim_end_matches(')')
                .split_whitespace();

            for path in paths {
                let expanded = shellexpand::tilde(path);
                entries.push(PathBuf::from(expanded.to_string()));
            }
        }
        
        // Handle multi-line path+=(...)
        let mut in_path_block = false;
        let mut path_entries = Vec::new();
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("path+=(") {
                in_path_block = true;
                continue;
            }
            
            if in_path_block {
                if trimmed == ")" {
                    in_path_block = false;
                    continue;
                }
                
                // Extract the path from quoted entries
                let path = trimmed
                    .trim_matches(|c| c == '"' || c == '\'' || c == ' ')
                    .to_string();
                
                if !path.is_empty() {
                    let expanded = shellexpand::tilde(&path);
                    path_entries.push(PathBuf::from(expanded.to_string()));
                }
            }
        }
        
        // Add path+= entries to our result
        entries.extend(path_entries);

        entries
    }

    fn format_path_export(&self, entries: &[PathBuf]) -> String {
        // Format in multi-line style to match common zsh configurations
        let paths = entries
            .iter()
            .map(|p| format!("  \"{}\"", p.to_string_lossy()))
            .collect::<Vec<_>>()
            .join("\n");

        // Use path+=() format for better compatibility with existing zsh configurations
        format!(
            "path+=(\n{}\n) # Updated by pathmaster on {}\n# Export PATH from path array\nexport PATH",
            paths,
            Local::now().format("%Y-%m-%d %H:%M:%S")
        )
    }

    fn detect_path_modifications(&self, content: &str) -> Vec<PathModification> {
        let mut modifications = self.find_path_arrays(content);

        // Look for standalone export PATH statements
        let path_regex = Regex::new(r"export PATH=").unwrap();
        for (idx, line) in content.lines().enumerate() {
            if path_regex.is_match(line) {
                modifications.push(PathModification {
                    line_number: idx + 1,
                    content: line.to_string(),
                    modification_type: ModificationType::Assignment,
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
            let lines: Vec<&str> = content.lines().collect();
            
            // Find the first path modification (which is where we'll update)
            let mut sorted_mods = modifications.clone();
            sorted_mods.sort_by(|a, b| a.line_number.cmp(&b.line_number));
            let first_mod = sorted_mods.first().unwrap().line_number - 1;
            
            // Create a vector of strings that we own
            let mut modified_lines = Vec::new();
            for line in &lines {
                modified_lines.push((*line).to_string());
            }
            
            // Use path+=() format to add to existing paths rather than replacing them
            let insert_pos = if first_mod + 1 < modified_lines.len() {
                // Insert after the path declaration
                first_mod + 1
            } else {
                // Insert at the end if we're at the last line
                first_mod
            };
            
            // Updated approach: Insert our new path+= section after the detected path= section
            // Split the new_path_config by lines and insert each line
            for (i, line) in new_path_config.lines().rev().enumerate() {
                modified_lines.insert(insert_pos, line.to_string());
            }
            
            // No longer comment out or replace the original declarations
            // This preserves the structure of the file better
            
            return modified_lines.join("\n");
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
    fn test_zsh_path_parsing() {
        let handler = ZshHandler::new();

        let content = r#"
# Some config
path=(/usr/bin /usr/local/bin ~/bin)
# Other config
"#;

        let entries = handler.parse_path_entries(content);
        assert_eq!(entries.len(), 3);
        assert!(entries.iter().any(|p| p.ends_with("usr/bin")));
        assert!(entries.iter().any(|p| p.ends_with("usr/local/bin")));
    }

    #[test]
    fn test_zsh_path_formatting() {
        let handler = ZshHandler::new();
        let entries = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];

        let formatted = handler.format_path_export(&entries);
        assert!(formatted.contains("path=("));
        assert!(formatted.contains(") && export PATH"));
        assert!(formatted.contains("/usr/bin"));
        assert!(formatted.contains("/usr/local/bin"));
    }

    #[test]
    fn test_zsh_config_update() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".zshrc");

        let initial_content = r#"
# Initial config
path=(/usr/bin /old/path)
export PATH="/another/old/path:$PATH"
"#;

        fs::write(&config_path, initial_content).unwrap();

        let mut handler = ZshHandler::new();
        handler.config_path = config_path.clone();

        let new_entries = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];

        handler.update_config(&new_entries).unwrap();

        let updated_content = fs::read_to_string(&config_path).unwrap();
        assert!(!updated_content.contains("/old/path"));
        assert!(updated_content.contains("/usr/bin"));
        assert!(updated_content.contains("/usr/local/bin"));
        assert!(updated_content.contains("path=("));
        assert!(updated_content.contains("export PATH"));
    }
    
    #[test]
    fn test_zsh_in_place_update() {
        let handler = ZshHandler::new();
        
        let content = r#"
# ZSH configuration
setopt AUTO_CD

# Shell options
HISTSIZE=1000
SAVEHIST=1000

# Path configuration
path=(/usr/bin /old/path /usr/sbin) && export PATH

# Aliases
alias ls='ls --color=auto'
"#;

        let new_entries = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];
        let updated_content = handler.update_path_in_config(content, &new_entries);
        
        // Verify the PATH was updated in-place and original config preserved
        let lines: Vec<&str> = updated_content.lines().collect();
        
        // Find where the original PATH declaration is in the updated content
        let original_path_line_index = lines.iter().position(|&line| 
            line.contains("path=(/usr/bin /old/path /usr/sbin)")).unwrap();
        
        // Find where our new path+=(...) declaration is
        let new_path_line_index = lines.iter().position(|&line| 
            line.contains("path+=(") && line.contains("Updated by pathmaster")).unwrap_or(0);
        
        // Check that original PATH is still present and in the correct position
        assert_eq!(original_path_line_index, 8, "Original PATH should remain at the same position");
        
        // Check that new PATH is right after the original path
        assert!(new_path_line_index > original_path_line_index, 
            "New PATH declaration should be after the original path declaration");
        
        // Check that PATH declarations are still between the shell options and aliases
        let histsize_line_index = lines.iter().position(|&line| line.contains("SAVEHIST=")).unwrap();
        let alias_line_index = lines.iter().position(|&line| line.contains("alias ls=")).unwrap();
        
        assert!(histsize_line_index < original_path_line_index, "PATH should be after SAVEHIST line");
        assert!(new_path_line_index < alias_line_index, "New PATH entries should be before alias line");
        
        // Check content
        assert!(updated_content.contains("/old/path"), "Original paths should be preserved");
        assert!(updated_content.contains("/usr/bin"));
        assert!(updated_content.contains("/usr/local/bin"));
        assert!(updated_content.contains("path+=("));
        assert!(updated_content.contains("# Updated by pathmaster on"));
    }
}