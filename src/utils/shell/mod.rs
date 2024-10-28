use std::io;
use std::path::PathBuf;

pub mod factory;
pub mod handlers;
pub mod types;

pub use self::handlers::ShellHandler;

pub fn update_shell_config(entries: &[PathBuf]) -> io::Result<()> {
    let handler = factory::get_shell_handler();
    handler.update_config(entries)
}
