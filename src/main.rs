//! Pathmaster - A powerful tool for managing your system's PATH environment variable.
//!
//! This binary provides a command-line interface for:
//! - Adding directories to PATH
//! - Removing directories from PATH
//! - Listing current PATH entries
//! - Managing PATH backups
//! - Validating PATH entries
//! - Flushing invalid entries from PATH

use clap::{command, Parser, Subcommand};
use commands::validator;

mod backup;
mod commands;
mod utils;

/// CLI configuration and argument parsing for pathmaster
#[derive(Parser)]
#[command(name = "pathmaster")]
#[command(version = "0.2.2")]
#[command(about = "A powerful path management tool", long_about = None)]
struct Cli {
    /// Control what gets backed up when modifying PATH (default, path, shell, switch)
    #[arg(long, value_name = "MODE")]
    backup_mode: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

/// Available commands for pathmaster
#[derive(Subcommand)]
enum Commands {
    /// Add directories to the PATH
    #[command(name = "add", short_flag = 'a')]
    Add {
        /// Directories to add
        directories: Vec<String>,
    },
    /// Delete directories from the PATH
    #[command(name = "delete", short_flag = 'd', aliases = &["remove"])]
    Delete {
        /// Directories to delete
        directories: Vec<String>,
    },
    /// List current PATH entries
    #[command(name = "list", short_flag = 'l')]
    List,
    /// Show backup history
    #[command(name = "history", short_flag = 'y')]
    History,
    /// Restore PATH from a backup
    #[command(name = "restore", short_flag = 'r')]
    Restore {
        /// Timestamp of the backup to restore
        #[arg(short, long)]
        timestamp: Option<String>,
    },
    /// Flush non-existing paths from the PATH
    #[command(name = "flush", short_flag = 'f')]
    Flush,
    /// Check PATH for invalid directories
    #[command(name = "check", short_flag = 'c')]
    Check,
}

fn main() {
    let cli = Cli::parse();

    // Initialize backup mode if specified
    if let Some(mode) = cli.backup_mode {
        let mut manager = backup::mode::BackupModeManager::new();
        match mode.as_str() {
            "default" => manager.reset_to_default(),
            "path" => manager.confirm_mode_change(backup::mode::BackupMode::PathOnly),
            "shell" => manager.confirm_mode_change(backup::mode::BackupMode::ShellOnly),
            "switch" => manager.toggle_mode(),
            _ => {
                eprintln!(
                    "Invalid backup mode: {}. Valid modes are: default, path, shell, switch",
                    mode
                );
                std::process::exit(1);
            }
        }
    }

    match &cli.command {
        Commands::Add { directories } => commands::add::execute(directories),
        Commands::Delete { directories } => commands::delete::execute(directories),
        Commands::List => commands::list::execute(),
        Commands::History => backup::show_history(),
        Commands::Restore { timestamp } => backup::restore_from_backup(timestamp),
        Commands::Flush => commands::flush::execute(),
        Commands::Check => match validator::validate_path() {
            Ok(validation) => {
                if validation.existing_dirs.is_empty() && validation.missing_dirs.is_empty() {
                    println!("All directories in PATH are valid");
                } else {
                    println!("Invalid directories in PATH:");
                    for dir in validation.missing_dirs {
                        println!("  {}", dir.to_string_lossy());
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
    }
}
