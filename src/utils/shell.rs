//! Shell configuration management functionality for the pathmaster tool.
//!
//! This module provides functionality for:
//! - Detecting and managing shell configuration files (.bashrc, .zshrc, .profile)
//! - Creating timestamped backups of shell configurations
//! - Updating PATH declarations in shell configuration files
//! - Supporting multiple shell types (bash, zsh, and generic shells)
//!
//! # Shell Support
//! - Bash (.bashrc)
//! - Zsh (.zshrc)
//! - Generic (.profile)
//!
//! # Example
//! ```rust
//! use pathmaster::utils::shell::ShellConfig;
//! use std::path::PathBuf;
//!
//! let shell_config = ShellConfig::new();
//! let paths = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];
//! shell_config.update_path(&paths).expect("Failed to update shell config");
//! ```

use chrono::Local;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents the different types of shells supported by pathmaster.
///
/// This enum is used to identify the shell type and determine which
/// configuration file should be modified when updating PATH.
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum ShellType {
    /// Bash shell, typically using .bashrc
    Bash,
    /// Zsh shell, typically using .zshrc
    Zsh,
    /// Generic shell, using .profile
    Generic,
}

/// Updates shell configuration files with the new PATH
///
/// # Arguments
/// * `entries` - Vector of PathBuf containing the new PATH entries
///
/// # Returns
/// * `Ok(())` if the update was successful
/// * `Err(std::io::Error)` if any file operations failed
pub fn update_shell_config(entries: &[PathBuf]) -> std::io::Result<()> {
    let shell = env::var("SHELL").unwrap_or_default();
    let home_dir = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("/"));

    let config_file = if shell.contains("bash") {
        home_dir.join(".bashrc")
    } else if shell.contains("zsh") {
        home_dir.join(".zshrc")
    } else {
        home_dir.join(".profile")
    };

    // Create backup of existing config
    let backup_path = config_file.with_extension("bak");
    if let Err(e) = fs::copy(&config_file, &backup_path) {
        eprintln!("Warning: Failed to create backup of shell config: {}", e);
        eprintln!("Backup would have been saved to: {}", backup_path.display());
    } else {
        println!(
            "Created backup of shell config at: {}",
            backup_path.display()
        );
    }

    // Read existing content
    let mut content = fs::read_to_string(&config_file)?;

    // Remove existing PATH exports
    content = content
        .lines()
        .filter(|line| !line.trim().starts_with("export PATH="))
        .collect::<Vec<&str>>()
        .join("\n");

    // Generate new PATH export command
    let new_path = env::join_paths(entries).unwrap();
    let export_command = format!(
        "\n# Updated PATH by pathmaster on {}\nexport PATH=\"{}\"\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        new_path.to_string_lossy()
    );

    // Write updated content
    std::fs::write(&config_file, content + &export_command)?;

    Ok(())
}

/// Manages shell configuration file operations and PATH updates.
///
/// This struct provides methods for:
/// - Creating backups of shell configuration files
/// - Updating PATH declarations
/// - Accessing shell configuration information
///
/// # Example
/// ```rust
/// # use pathmaster::utils::shell::ShellConfig;
/// let config = ShellConfig::new();
/// let backup_path = config.create_backup().expect("Failed to create backup");
/// println!("Backup created at: {}", backup_path.display());
/// ```

#[allow(dead_code)]
pub struct ShellConfig {
    /// The type of shell detected
    shell_type: ShellType,
    /// Path to the shell's configuration file
    config_path: PathBuf,
}
#[allow(dead_code)]
impl ShellConfig {
    /// Creates a new ShellConfig instance by detecting the current shell environment.
    ///
    /// This method:
    /// 1. Detects the current shell type from the SHELL environment variable
    /// 2. Determines the appropriate configuration file path
    /// 3. Creates a ShellConfig instance with the detected information
    ///
    /// # Returns
    /// A new ShellConfig instance configured for the detected shell
    ///
    /// # Example
    /// ```rust
    /// # use pathmaster::utils::shell::ShellConfig;
    /// let config = ShellConfig::new();
    /// ```
    pub fn new() -> Self {
        let (shell_type, config_path) = detect_shell_config();
        Self {
            shell_type,
            config_path,
        }
    }

    /// Returns the path to the shell's configuration file.
    ///
    /// # Returns
    /// A reference to the PathBuf containing the configuration file path
    ///
    /// # Example
    /// ```rust
    /// # use pathmaster::utils::shell::ShellConfig;
    /// let config = ShellConfig::new();
    /// println!("Config file: {}", config.config_path().display());
    /// ```
    pub fn config_path(&self) -> &Path {
        &self.config_path
    }

    /// Creates a timestamped backup of the current shell configuration file.
    ///
    /// The backup file will be created with a .bak_TIMESTAMP extension, where
    /// TIMESTAMP is in the format YYYYMMDDHHMMSS.
    ///
    /// # Returns
    /// * `Ok(PathBuf)` - Path to the created backup file
    /// * `Err(std::io::Error)` - If backup creation fails
    ///
    /// # Example
    /// ```rust
    /// # use pathmaster::utils::shell::ShellConfig;
    /// let config = ShellConfig::new();
    /// match config.create_backup() {
    ///     Ok(backup_path) => println!("Backup created at: {}", backup_path.display()),
    ///     Err(e) => eprintln!("Failed to create backup: {}", e),
    /// }
    /// ```
    pub fn create_backup(&self) -> std::io::Result<PathBuf> {
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let backup_path = self
            .config_path
            .with_extension(format!("bak_{}", timestamp));

        fs::copy(&self.config_path, &backup_path)?;
        Ok(backup_path)
    }

    /// Updates the PATH declaration in the shell configuration file.
    ///
    /// This method:
    /// 1. Creates a backup of the current configuration
    /// 2. Removes any existing PATH exports
    /// 3. Adds a new PATH export with the provided entries
    ///
    /// # Arguments
    /// * `entries` - Vector of PathBuf containing the desired PATH entries
    ///
    /// # Returns
    /// * `Ok(())` if the update was successful
    /// * `Err(std::io::Error)` if any file operations failed
    ///
    /// # Example
    /// ```rust
    /// # use pathmaster::utils::shell::ShellConfig;
    /// # use std::path::PathBuf;
    /// let config = ShellConfig::new();
    /// let paths = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];
    /// match config.update_path(&paths) {
    ///     Ok(_) => println!("PATH updated successfully"),
    ///     Err(e) => eprintln!("Failed to update PATH: {}", e),
    /// }
    /// ```
    pub fn update_path(&self, entries: &[PathBuf]) -> std::io::Result<()> {
        // Create backup before modification
        let backup_path = self.create_backup()?;
        println!(
            "Created backup of shell config at: {}",
            backup_path.display()
        );

        // Read existing content
        let mut content = fs::read_to_string(&self.config_path)?;

        // Remove existing PATH exports
        content = content
            .lines()
            .filter(|line| !line.trim().starts_with("export PATH="))
            .collect::<Vec<&str>>()
            .join("\n");

        // Generate new PATH export command
        let new_path = env::join_paths(entries).unwrap();
        let export_command = format!(
            "\n# Updated PATH by pathmaster on {}\nexport PATH=\"{}\"\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            new_path.to_string_lossy()
        );

        // Write updated content
        fs::write(&self.config_path, content + &export_command)?;

        Ok(())
    }

    /// Returns the detected shell type.
    ///
    /// # Returns
    /// A reference to the ShellType enum indicating the current shell
    ///
    /// # Example
    /// ```rust
    /// # use pathmaster::utils::shell::ShellConfig;
    /// let config = ShellConfig::new();
    /// println!("Current shell type: {:?}", config.shell_type());
    /// ```
    pub fn shell_type(&self) -> &ShellType {
        &self.shell_type
    }
}

/// Detects the current shell type and its configuration file path.
///
/// This function:
/// 1. Checks the SHELL environment variable
/// 2. Determines the shell type (bash, zsh, or generic)
/// 3. Locates the appropriate configuration file in the home directory
///
/// # Returns
/// A tuple containing:
/// * The detected ShellType
/// * PathBuf to the shell's configuration file
///
/// # Note
/// If the SHELL environment variable is not set or the home directory
/// cannot be determined, defaults to Generic shell type with .profile
pub fn detect_shell_config() -> (ShellType, PathBuf) {
    let shell = env::var("SHELL").unwrap_or_default();
    let home_dir = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("/"));

    if shell.contains("bash") {
        (ShellType::Bash, home_dir.join(".bashrc"))
    } else if shell.contains("zsh") {
        (ShellType::Zsh, home_dir.join(".zshrc"))
    } else {
        (ShellType::Generic, home_dir.join(".profile"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn test_shell_detection() {
        // Test with bash
        env::set_var("SHELL", "/bin/bash");
        let (shell_type, _) = detect_shell_config();
        assert_eq!(shell_type, ShellType::Bash);

        // Test with zsh
        env::set_var("SHELL", "/bin/zsh");
        let (shell_type, _) = detect_shell_config();
        assert_eq!(shell_type, ShellType::Zsh);

        // Test with unknown shell
        env::set_var("SHELL", "/bin/other");
        let (shell_type, _) = detect_shell_config();
        assert_eq!(shell_type, ShellType::Generic);
    }

    #[test]
    fn test_shell_config_backup() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".bashrc");

        // Create dummy config file
        File::create(&config_path).unwrap();

        let shell_config = ShellConfig {
            shell_type: ShellType::Bash,
            config_path: config_path.clone(),
        };

        let backup_result = shell_config.create_backup();
        assert!(backup_result.is_ok());

        let backup_path = backup_result.unwrap();
        assert!(backup_path.exists());
        assert!(backup_path.to_string_lossy().contains("bak_"));
    }

    #[test]
    fn test_path_update() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".bashrc");

        // Create dummy config file with existing PATH
        fs::write(&config_path, "export PATH=/old/path\n").unwrap();

        let shell_config = ShellConfig {
            shell_type: ShellType::Bash,
            config_path: config_path.clone(),
        };

        let new_paths = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];

        let update_result = shell_config.update_path(&new_paths);
        assert!(update_result.is_ok());

        // Verify content was updated
        let content = fs::read_to_string(&config_path).unwrap();
        assert!(!content.contains("export PATH=/old/path"));
        assert!(content.contains("export PATH=\""));
        assert!(content.contains("/usr/bin"));
        assert!(content.contains("/usr/local/bin"));
    }
}
