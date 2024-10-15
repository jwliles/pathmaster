use clap::{Parser, Subcommand};

mod backup;
mod commands;
mod utils;

#[derive(Parser)]
#[command(name = "pathfinder")]
#[command(version = "0.1.0")]
#[command(about = "A powerful path management tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

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
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { directories } => commands::add::execute(directories),
        Commands::Delete { directories } => commands::delete::execute(directories),
        Commands::List => commands::list::execute(),
        Commands::History => backup::show_history(),
        Commands::Restore { timestamp } => commands::restore::execute(timestamp),
        Commands::Flush => commands::flush::execute(),
    }
}
