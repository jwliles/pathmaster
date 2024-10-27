//! Utility modules for pathmaster functionality.

pub mod path;
pub mod path_scanner;
pub mod shell;

// Re-export commonly used functionality
pub use path::{expand_path, get_path_entries, set_path_entries};
pub use shell::update_shell_config;
