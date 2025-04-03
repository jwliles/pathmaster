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
            for line in new_path_config.lines().rev() {
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
    fn test_zsh_multiline_path_parsing() {
        let handler = ZshHandler::new();

        let content = r#"
# Some config
path+=(
  "$HOME/bin"
  "$HOME/.local/bin"
  "/usr/local/bin"
)
# Other config
"#;

        let entries = handler.parse_path_entries(content);
        assert_eq!(entries.len(), 3);
        assert!(entries.iter().any(|p| p.to_string_lossy().contains("bin")));
        assert!(entries.iter().any(|p| p.to_string_lossy().contains(".local/bin")));
        assert!(entries.iter().any(|p| p.to_string_lossy().contains("/usr/local/bin")));
    }

    #[test]
    fn test_zsh_path_formatting() {
        let handler = ZshHandler::new();
        let entries = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];

        let formatted = handler.format_path_export(&entries);
        assert!(formatted.contains("path+=("));
        assert!(formatted.contains("export PATH"));
        assert!(formatted.contains("/usr/bin"));
        assert!(formatted.contains("/usr/local/bin"));
        // Check for multi-line format
        assert!(formatted.contains("\n"));
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
        // Now we preserve old path entries
        assert!(updated_content.contains("/old/path"));
        assert!(updated_content.contains("/usr/bin"));
        assert!(updated_content.contains("/usr/local/bin"));
        assert!(updated_content.contains("path+=("));
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
        
        // Verify that the original content is preserved
        assert!(updated_content.contains("# ZSH configuration"));
        assert!(updated_content.contains("setopt AUTO_CD"));
        assert!(updated_content.contains("HISTSIZE=1000"));
        
        // Verify that PATH-related sections are preserved and updated
        assert!(updated_content.contains("path=(/usr/bin /old/path /usr/sbin)"), 
                "Original path declaration should be preserved");
        assert!(updated_content.contains("path+=("), 
                "New path+=() declaration should be added");
        
        // Check that PATH declarations are in the right order
        let lines: Vec<&str> = updated_content.lines().collect();
        let original_path_index = lines.iter().position(|&line| 
            line.contains("path=(/usr/bin /old/path /usr/sbin)")).unwrap();
        let alias_index = lines.iter().position(|&line| 
            line.contains("alias ls='ls --color=auto'")).unwrap();
            
        // Check that the original path declaration is before the aliases
        assert!(original_path_index < alias_index, 
                "PATH declarations should be before aliases");
                
        // Check content
        assert!(updated_content.contains("/old/path"), "Original paths should be preserved");
        assert!(updated_content.contains("/usr/bin"));
        assert!(updated_content.contains("/usr/local/bin"));
        assert!(updated_content.contains("# Updated by pathmaster on"));
    }
    
    #[test]
    fn test_preserve_first_line() {
        let handler = ZshHandler::new();
        
        // This specifically tests for the bug where the first line would be missing
        let content = r#"export ZSH="$HOME/.oh-my-zsh"

# Initialize the path array with unique entries
typeset -U path

# Append directories to the path array
path+=(
  "$HOME/Applications"
  "$HOME/bin"
)

# Export PATH from path array
export PATH"#;

        let new_entries = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];
        let updated_content = handler.update_path_in_config(content, &new_entries);
        
        // The first line should still be there
        let first_line = updated_content.lines().next().unwrap();
        assert!(first_line.contains("export ZSH="), "First line should be preserved");
        
        // The path+=() block should still be there
        assert!(updated_content.contains("path+=("), "Original path+=() should be preserved");
        assert!(updated_content.contains("$HOME/Applications"), "Original path entries should be preserved");
        
        // Our new path+=() should be added
        assert!(updated_content.contains("/usr/bin"));
        assert!(updated_content.contains("/usr/local/bin"));
    }
    
    #[test]
    fn test_multiline_path_structure_preservation() {
        let handler = ZshHandler::new();
        
        // This tests preservation of multi-line path+= structure
        let content = r#"# Initialize the path array with unique entries
typeset -U path

# Append directories to the path array
path+=(
  "$HOME/Applications"
  "$HOME/Applications/bin"
  "$HOME/Applications/scripts"
  "$HOME/.local/bin"
)

# Export PATH from path array
export PATH"#;

        let new_entries = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];
        let updated_content = handler.update_path_in_config(content, &new_entries);
        
        // The original structure should be preserved
        assert!(updated_content.contains("typeset -U path"), "typeset -U path should be preserved");
        assert!(updated_content.contains("# Append directories to the path array"), "Comments should be preserved");
        assert!(updated_content.contains("$HOME/Applications"), "Original path entries should be preserved");
        
        // Verify that our update contains the new paths
        assert!(updated_content.contains("/usr/bin"), "New path entries should be added");
        assert!(updated_content.contains("/usr/local/bin"), "New path entries should be added");
        
        // Verify multi-line format (rather than checking exact format which might change)
        assert!(updated_content.contains("path+=("), "Should use path+=( format");
        assert!(updated_content.lines().count() > content.lines().count(), 
               "Updated content should have more lines due to multi-line path format");
    }
    
    #[test]
    fn test_real_world_oh_my_zsh_config() {
        let handler = ZshHandler::new();
        
        // This simulates a real .zshrc from oh-my-zsh with the missing first line issue
        let content = r#"export ZSH="$HOME/.oh-my-zsh"

# Initialize the path array with unique entries
typeset -U path

# Append directories to the path array
path+=(
  "$HOME/Applications"
  "$HOME/Applications/bin"
  "$HOME/Applications/scripts"
  "$HOME/.local/bin"
)

# Ensure unique path entries
typeset -U path

# Export PATH from path array
export PATH

# Set oh-my-zsh update mode
zstyle ':omz:update' mode auto # update automatically without asking"#;

        let new_entries = vec![
            PathBuf::from("/usr/bin"), 
            PathBuf::from("/usr/local/bin"),
            PathBuf::from("/usr/local/sbin"),
        ];
        
        let updated_content = handler.update_path_in_config(content, &new_entries);
        
        // Test for first line preservation
        let first_line = updated_content.lines().next().unwrap();
        assert_eq!(first_line, "export ZSH=\"$HOME/.oh-my-zsh\"", "First line should be exactly preserved");
        
        // Test for multi-line path+= section preservation
        assert!(updated_content.contains("path+=("), "Original path+=() should be preserved");
        assert!(updated_content.contains("$HOME/Applications"), "Original path entries should be preserved");
        
        // Test for our new path+=() section with proper multi-line format
        assert!(updated_content.contains("\"/usr/bin\""));
        assert!(updated_content.contains("\"/usr/local/bin\""));
        assert!(updated_content.contains("\"/usr/local/sbin\""));
        
        // Test that all original sections are preserved
        assert!(updated_content.contains("zstyle ':omz:update' mode auto"), 
                "Content after PATH sections should be preserved");
    }
}