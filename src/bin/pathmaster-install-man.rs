use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    // Get the path to the man page
    let man_page = env::var("PATHMASTER_MAN_PAGE").unwrap_or_else(|_| {
        eprintln!("PATHMASTER_MAN_PAGE environment variable not set");
        process::exit(1);
    });

    // Get the installation prefix
    let prefix = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: pathmaster-install-man <prefix>");
        process::exit(1);
    });

    // Create the man directory
    let man_dir = Path::new(&prefix).join("share").join("man").join("man1");
    fs::create_dir_all(&man_dir).unwrap_or_else(|e| {
        eprintln!("Failed to create directory {}: {}", man_dir.display(), e);
        process::exit(1);
    });

    // Copy the man page
    let dest = man_dir.join("pathmaster.1");
    fs::copy(&man_page, &dest).unwrap_or_else(|e| {
        eprintln!("Failed to copy {} to {}: {}", man_page, dest.display(), e);
        process::exit(1);
    });

    println!("Installed man page to {}", dest.display());
}
