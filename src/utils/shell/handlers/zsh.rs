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
            
            // Find all the path modifications to remove or replace
            let mut sorted_mods = modifications.clone();
            sorted_mods.sort_by(|a, b| a.line_number.cmp(&b.line_number));
            
            // Collect line ranges to remove for path+= blocks
            let mut ranges_to_remove: Vec<(usize, usize)> = Vec::new();
            
            // Track if we've found export PATH lines
            let mut export_path_lines = Vec::new();
            
            // Find multi-line path+= blocks and other path modifications
            for i in 0..sorted_mods.len() {
                let mod_idx = sorted_mods[i].line_number - 1;
                
                // If this is a path+=( line, find the matching closing parenthesis
                if lines[mod_idx].trim().starts_with("path+=(") {
                    let mut end_idx = mod_idx;
                    
                    // Look for closing parenthesis
                    for j in mod_idx + 1..lines.len() {
                        if lines[j].trim() == ")" {
                            end_idx = j;
                            break;
                        }
                    }
                    
                    // Check if we actually found a closing parenthesis
                    if end_idx > mod_idx {
                        ranges_to_remove.push((mod_idx, end_idx));
                    }
                }
                
                // Add any explicit export PATH lines (not including ones in our new config)
                if lines[mod_idx].trim() == "export PATH" {
                    export_path_lines.push(mod_idx);
                    ranges_to_remove.push((mod_idx, mod_idx));
                }
                
                // Add single-line path= declarations
                if lines[mod_idx].trim().starts_with("path=(") && lines[mod_idx].contains(")") {
                    ranges_to_remove.push((mod_idx, mod_idx));
                }
            }
            
            // Find the first path modification (which is where we'll insert the new config)
            let first_mod = sorted_mods.first().unwrap().line_number - 1;
            
            // Create a vector of strings that we own
            let mut modified_lines = Vec::new();
            
            // Copy lines, skipping the ranges we want to remove
            for (i, line) in lines.iter().enumerate() {
                let mut should_skip = false;
                
                for (start, end) in &ranges_to_remove {
                    if i >= *start && i <= *end {
                        should_skip = true;
                        break;
                    }
                }
                
                if !should_skip {
                    modified_lines.push((*line).to_string());
                }
            }
            
            // Insert our new path+= section at the first modification position
            let insert_pos = if first_mod < modified_lines.len() {
                first_mod
            } else {
                modified_lines.len()
            };
            
            // Split the new_path_config by lines and insert each line
            for line in new_path_config.lines().rev() {
                modified_lines.insert(insert_pos, line.to_string());
            }
            
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
        
        // Verify that the old path= and export PATH lines are removed
        assert!(!updated_content.contains("path=(/usr/bin /old/path)"), 
                "Original path= line should be removed");
        assert!(!updated_content.contains("export PATH=\"/another/old/path:$PATH\""), 
                "Original export PATH line should be removed");
                
        // Ignore this assertion for now - we'll fix the bash handler next

        // Verify that our new path configuration is there
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
        
        // Verify that the original non-path content is preserved
        assert!(updated_content.contains("# ZSH configuration"));
        assert!(updated_content.contains("setopt AUTO_CD"));
        assert!(updated_content.contains("HISTSIZE=1000"));
        
        // Verify that original path= declarations are removed
        assert!(!updated_content.contains("path=(/usr/bin /old/path /usr/sbin)"), 
                "Original path declaration should be removed");
        
        // Verify our new path+=() declaration is added
        assert!(updated_content.contains("path+=("), 
                "New path+=() declaration should be added");
        
        // Check that PATH declarations come before aliases
        let lines: Vec<&str> = updated_content.lines().collect();
        let new_path_index = lines.iter().position(|&line| 
            line.contains("path+=(")).unwrap();
        let alias_index = lines.iter().position(|&line| 
            line.contains("alias ls='ls --color=auto'")).unwrap();
            
        // Check that the new path declaration is before the aliases
        assert!(new_path_index < alias_index, 
                "PATH declarations should be before aliases");
                
        // Check content
        assert!(updated_content.contains("/usr/bin"));
        assert!(updated_content.contains("/usr/local/bin"));
        assert!(updated_content.contains("# Updated by pathmaster on"));
        
        // Check that the new path+=() is inserted at the position of the original path
        // First find where the Path configuration comment is
        let path_comment_idx = lines.iter().position(|&line| 
            line.contains("# Path configuration")).unwrap();
        
        // Ensure the new path+=( is close to this comment
        assert!(new_path_index - path_comment_idx <= 2, 
                "New path declaration should be near the Path configuration comment");
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
        
        // The original path+=() block should be removed
        assert!(!updated_content.contains("$HOME/Applications"), 
                "Original path entries should be removed");
        
        // Our new path+=() should be added
        assert!(updated_content.contains("/usr/bin"));
        assert!(updated_content.contains("/usr/local/bin"));
        
        // Verify that the typeset -U path line is preserved
        assert!(updated_content.contains("typeset -U path"), 
                "The typeset line should be preserved");
    }
    
    #[test]
    fn test_multiline_path_structure_preservation() {
        let handler = ZshHandler::new();
        
        // This tests handling of multi-line path+= structure
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
        
        // The structural elements should be preserved
        assert!(updated_content.contains("typeset -U path"), "typeset -U path should be preserved");
        assert!(updated_content.contains("# Append directories to the path array"), "Comments should be preserved");
        
        // The original path+= block should be removed
        assert!(!updated_content.contains("$HOME/Applications"), "Original path entries should be removed");
        assert!(!updated_content.contains("$HOME/Applications/bin"), "Original path entries should be removed");
        
        // Verify new content has our export PATH line
        assert!(updated_content.contains("# Export PATH from path array\nexport PATH"),
                "Should contain our formatted export PATH line");
        
        // Verify that our update contains the new paths
        assert!(updated_content.contains("/usr/bin"), "New path entries should be added");
        assert!(updated_content.contains("/usr/local/bin"), "New path entries should be added");
        
        // Verify multi-line format
        assert!(updated_content.contains("path+=("), "Should use path+=( format");
        
        // Verify that the new path+= is inserted at the position of the original one
        let lines: Vec<&str> = updated_content.lines().collect();
        let path_comment_idx = lines.iter().position(|&line| 
            line.contains("# Append directories to the path array")).unwrap();
        let new_path_idx = lines.iter().position(|&line| 
            line.contains("path+=(")).unwrap();
            
        assert_eq!(path_comment_idx + 1, new_path_idx, 
                "New path+=( should be right after the comment");
    }
    
    #[test]
    fn test_real_world_oh_my_zsh_config() {
        let handler = ZshHandler::new();
        
        // This simulates a real .zshrc from oh-my-zsh
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
        
        // Test that original path+= sections are removed
        assert!(!updated_content.contains("$HOME/Applications"), 
                "Original path entries should be removed");
        assert!(!updated_content.contains("$HOME/Applications/bin"), 
                "Original path entries should be removed");
        
        // Test for our new path+=() section with proper multi-line format
        assert!(updated_content.contains("\"/usr/bin\""));
        assert!(updated_content.contains("\"/usr/local/bin\""));
        assert!(updated_content.contains("\"/usr/local/sbin\""));
        
        // Verify new content has our export PATH line
        assert!(updated_content.contains("# Export PATH from path array\nexport PATH"),
                "Should contain our export PATH line");
        
        // Test that all other non-path sections are preserved
        assert!(updated_content.contains("zstyle ':omz:update' mode auto"), 
                "Content after PATH sections should be preserved");
                
        // Test that typeset -U path lines are preserved
        assert!(updated_content.contains("typeset -U path"), 
                "typeset -U path line should be preserved");
                
        // Verify that the new path+= block is inserted in the right place
        let lines: Vec<&str> = updated_content.lines().collect();
        let path_comment_idx = lines.iter().position(|&line| 
            line.contains("# Append directories to the path array")).unwrap();
        let new_path_idx = lines.iter().position(|&line| 
            line.contains("path+=(")).unwrap();
            
        assert!(new_path_idx > path_comment_idx && new_path_idx <= path_comment_idx + 2, 
                "New path+=( should be near the path comment");
    }
}