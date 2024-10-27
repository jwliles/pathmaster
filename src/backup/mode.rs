//! Backup mode management for the pathmaster tool.
//!
//! This module handles:
//! - Backup mode configuration (PATH-only, shell-only, or both)
//! - Mode switching and resetting
//! - Mode conflict resolution
//! - Mode persistence

use std::fmt;
use std::str::FromStr;

/// Represents available backup modes for pathmaster.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BackupMode {
    /// Backs up both PATH and shell configurations (default)
    Both,
    /// Backs up only PATH entries
    PathOnly,
    /// Backs up only shell configuration
    ShellOnly,
}

impl Default for BackupMode {
    fn default() -> Self {
        Self::Both
    }
}

impl fmt::Display for BackupMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackupMode::Both => write!(f, "both"),
            BackupMode::PathOnly => write!(f, "path"),
            BackupMode::ShellOnly => write!(f, "shell"),
        }
    }
}

impl FromStr for BackupMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "both" => Ok(BackupMode::Both),
            "path" => Ok(BackupMode::PathOnly),
            "shell" => Ok(BackupMode::ShellOnly),
            _ => Err(format!("Invalid backup mode: {}", s)),
        }
    }
}
#[allow(dead_code)]
impl BackupMode {
    /// Returns whether PATH should be backed up in this mode
    pub fn should_backup_path(&self) -> bool {
        matches!(self, BackupMode::Both | BackupMode::PathOnly)
    }

    /// Returns whether shell config should be backed up in this mode
    pub fn should_backup_shell(&self) -> bool {
        matches!(self, BackupMode::Both | BackupMode::ShellOnly)
    }

    /// Toggles between PathOnly and ShellOnly modes
    pub fn toggle(&self) -> Self {
        match self {
            BackupMode::Both => BackupMode::PathOnly,
            BackupMode::PathOnly => BackupMode::ShellOnly,
            BackupMode::ShellOnly => BackupMode::PathOnly,
        }
    }
}

/// Represents the result of attempting to change backup modes
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum ModeChangeResult {
    /// Mode was changed successfully
    Changed(BackupMode),
    /// Requires user confirmation due to conflict
    NeedsConfirmation {
        current: BackupMode,
        requested: BackupMode,
    },
}

/// Manages backup mode state and transitions
#[derive(Debug)]
#[allow(dead_code)]
pub struct BackupModeManager {
    current_mode: BackupMode,
}

impl Default for BackupModeManager {
    fn default() -> Self {
        Self {
            current_mode: BackupMode::default(),
        }
    }
}
#[allow(dead_code)]
impl BackupModeManager {
    /// Creates a new BackupModeManager with default mode
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the current backup mode
    pub fn current_mode(&self) -> BackupMode {
        self.current_mode
    }

    /// Attempts to change to a new backup mode
    ///
    /// # Returns
    /// - `ModeChangeResult::Changed` if the mode was changed
    /// - `ModeChangeResult::NeedsConfirmation` if there's a conflict
    pub fn request_mode_change(&self, new_mode: BackupMode) -> ModeChangeResult {
        if self.current_mode == new_mode {
            return ModeChangeResult::Changed(new_mode);
        }

        if self.current_mode == BackupMode::Both {
            ModeChangeResult::Changed(new_mode)
        } else {
            ModeChangeResult::NeedsConfirmation {
                current: self.current_mode,
                requested: new_mode,
            }
        }
    }

    /// Switches to a new mode after confirmation
    pub fn confirm_mode_change(&mut self, new_mode: BackupMode) {
        self.current_mode = new_mode;
    }

    /// Resets to default mode (Both)
    pub fn reset_to_default(&mut self) {
        self.current_mode = BackupMode::default();
    }

    /// Toggles between PathOnly and ShellOnly modes
    pub fn toggle_mode(&mut self) {
        self.current_mode = self.current_mode.toggle();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_mode_defaults() {
        assert_eq!(BackupMode::default(), BackupMode::Both);
        assert_eq!(BackupModeManager::new().current_mode(), BackupMode::Both);
    }

    #[test]
    fn test_backup_flags() {
        let mode = BackupMode::Both;
        assert!(mode.should_backup_path());
        assert!(mode.should_backup_shell());

        let mode = BackupMode::PathOnly;
        assert!(mode.should_backup_path());
        assert!(!mode.should_backup_shell());

        let mode = BackupMode::ShellOnly;
        assert!(!mode.should_backup_path());
        assert!(mode.should_backup_shell());
    }

    #[test]
    fn test_mode_toggle() {
        assert_eq!(BackupMode::Both.toggle(), BackupMode::PathOnly);
        assert_eq!(BackupMode::PathOnly.toggle(), BackupMode::ShellOnly);
        assert_eq!(BackupMode::ShellOnly.toggle(), BackupMode::PathOnly);
    }

    #[test]
    fn test_mode_change_requests() {
        let manager = BackupModeManager::new();

        // From Both to PathOnly should work directly
        match manager.request_mode_change(BackupMode::PathOnly) {
            ModeChangeResult::Changed(mode) => assert_eq!(mode, BackupMode::PathOnly),
            _ => panic!("Expected direct mode change"),
        }

        // Change manager to PathOnly for next test
        let mut manager = BackupModeManager::new();
        manager.confirm_mode_change(BackupMode::PathOnly);

        // From PathOnly to ShellOnly should need confirmation
        match manager.request_mode_change(BackupMode::ShellOnly) {
            ModeChangeResult::NeedsConfirmation { current, requested } => {
                assert_eq!(current, BackupMode::PathOnly);
                assert_eq!(requested, BackupMode::ShellOnly);
            }
            _ => panic!("Expected confirmation request"),
        }
    }

    #[test]
    fn test_mode_parsing() {
        assert_eq!("both".parse::<BackupMode>().unwrap(), BackupMode::Both);
        assert_eq!("path".parse::<BackupMode>().unwrap(), BackupMode::PathOnly);
        assert_eq!(
            "shell".parse::<BackupMode>().unwrap(),
            BackupMode::ShellOnly
        );
        assert!("invalid".parse::<BackupMode>().is_err());
    }

    #[test]
    fn test_mode_display() {
        assert_eq!(BackupMode::Both.to_string(), "both");
        assert_eq!(BackupMode::PathOnly.to_string(), "path");
        assert_eq!(BackupMode::ShellOnly.to_string(), "shell");
    }
}
