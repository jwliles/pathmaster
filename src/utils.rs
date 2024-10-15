use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub fn expand_path(path: &str) -> PathBuf {
    let expanded = shellexpand::tilde(path);
    PathBuf::from(expanded.to_string())
}

pub fn get_path_entries() -> Vec<PathBuf> {
    env::var_os("PATH")
        .map(|paths| env::split_paths(&paths).collect())
        .unwrap_or_default()
}

pub fn set_path_entries(entries: &[PathBuf]) {
    let new_path = env::join_paths(entries).expect("Failed to join PATH entries");
    env::set_var("PATH", &new_path);
}

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

    let new_path = env::join_paths(entries).unwrap();
    let export_command = format!(
        "\n# Updated PATH by pathfinder\nexport PATH=\"{}\"\n",
        new_path.to_string_lossy()
    );

    let mut file = OpenOptions::new().append(true).open(config_file)?;
    file.write_all(export_command.as_bytes())?;

    Ok(())
}
