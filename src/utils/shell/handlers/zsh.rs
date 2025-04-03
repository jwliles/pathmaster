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
        let path_array_regex = Regex::new(r"(?m)^path=\((.*?)\)").unwrap();

        path_array_regex
            .captures_iter(content)
            .enumerate()
            .map(|(idx, cap)| PathModification {
                line_number: idx + 1,
                content: cap[0].to_string(),
                modification_type: ModificationType::ArrayModification,
            })
            .collect()
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

        entries
    }

    fn format_path_export(&self, entries: &[PathBuf]) -> String {
        let paths = entries
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(" ");

        format!(
            "\n# Updated by pathmaster on {}\npath=({}) && export PATH\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            paths
        )
    }

    fn detect_path_modifications(&self, content: &str) -> Vec<PathModification> {
        let mut modifications = self.find_path_arrays(content);

        let path_regex = Regex::new(r"(?m)^export PATH=").unwrap();
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
            // Remove newline prefix from format_path_export output
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
        
        // Verify the PATH was updated in-place
        let lines: Vec<&str> = updated_content.lines().collect();
        
        // Find where the PATH declaration is in the updated content
        let mut path_line_index = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.contains("path=(") {
                path_line_index = i;
                break;
            }
        }
        
        // Check that PATH is still between the shell options and aliases
        let histsize_line_index = lines.iter().position(|&line| line.contains("SAVEHIST=")).unwrap();
        let alias_line_index = lines.iter().position(|&line| line.contains("alias ls=")).unwrap();
        
        assert!(histsize_line_index < path_line_index, "PATH should be after SAVEHIST line");
        assert!(path_line_index < alias_line_index, "PATH should be before alias line");
        
        // Check content
        assert!(!updated_content.contains("/old/path"));
        assert!(updated_content.contains("/usr/bin"));
        assert!(updated_content.contains("/usr/local/bin"));
        assert!(updated_content.contains("path=("));
    }
}