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

        let updated_content = content
            .lines()
            .enumerate()
            .filter(|(idx, line)| {
                !modifications.iter().any(|m| m.line_number == idx + 1)
                    && !line.contains("/old/path") // Explicitly filter out old paths
            })
            .map(|(_, line)| line)
            .collect::<Vec<_>>()
            .join("\n");

        // Add new PATH configuration
        updated_content + &self.format_path_export(entries)
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
}
