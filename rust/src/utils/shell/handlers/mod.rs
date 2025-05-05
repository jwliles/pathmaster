use chrono::Local;
use std::fs;
use std::io;
use std::path::PathBuf;

pub mod bash;
pub mod fish;
pub mod generic;
pub mod ksh;
pub mod tcsh;
pub mod zsh;

pub use bash::BashHandler;
pub use fish::FishHandler;
pub use generic::GenericHandler;
pub use ksh::KshHandler;
pub use tcsh::TcshHandler;
pub use zsh::ZshHandler;

use crate::utils::shell::types::*;

#[allow(dead_code)]
pub trait ShellHandler {
    fn get_shell_type(&self) -> ShellType;
    fn get_config_path(&self) -> PathBuf;
    fn parse_path_entries(&self, content: &str) -> Vec<PathBuf>;
    fn format_path_export(&self, entries: &[PathBuf]) -> String;
    fn detect_path_modifications(&self, content: &str) -> Vec<PathModification>;
    fn update_path_in_config(&self, content: &str, entries: &[PathBuf]) -> String;

    fn create_backup(&self) -> io::Result<PathBuf> {
        let config_path = self.get_config_path();
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let backup_path = config_path.with_extension(format!("bak_{}", timestamp));

        fs::copy(&config_path, &backup_path)?;
        Ok(backup_path)
    }

    fn update_config(&self, entries: &[PathBuf]) -> io::Result<()> {
        let config_path = self.get_config_path();
        let backup_path = self.create_backup()?;
        println!(
            "Created backup of shell config at: {}",
            backup_path.display()
        );

        let content = fs::read_to_string(&config_path)?;
        let updated_content = self.update_path_in_config(&content, entries);
        fs::write(&config_path, updated_content)?;

        Ok(())
    }
}
