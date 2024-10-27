use regex::Regex;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug)]
#[allow(dead_code)]
pub struct PathLocation {
    file: PathBuf,
    line_number: usize,
    content: String,
    requires_sudo: bool,
}

#[allow(dead_code)]
pub struct PathScanner {
    path_regex: Regex,
}

#[allow(dead_code)]
impl PathScanner {
    pub fn new() -> Self {
        let path_regex = Regex::new(r"(PATH=|export PATH|setenv PATH|path\+=)").unwrap();
        Self { path_regex }
    }

    pub fn scan_all(&self) -> io::Result<Vec<PathLocation>> {
        let mut results = Vec::new();

        // System-level files (requires sudo)
        let system_files = self.get_system_files()?;
        for file in system_files {
            if let Ok(mut locations) = self.scan_file(&file, true) {
                results.append(&mut locations);
            }
        }

        // User-level files
        let user_files = self.get_user_files()?;
        for file in user_files {
            if let Ok(mut locations) = self.scan_file(&file, false) {
                results.append(&mut locations);
            }
        }

        Ok(results)
    }

    fn get_system_files(&self) -> io::Result<Vec<PathBuf>> {
        let mut files = vec![
            PathBuf::from("/etc/environment"),
            PathBuf::from("/etc/profile"),
            PathBuf::from("/etc/bash.bashrc"),
            PathBuf::from("/etc/bashrc"),
        ];

        // Add all scripts from /etc/profile.d/
        if let Ok(entries) = fs::read_dir("/etc/profile.d") {
            for entry in entries.flatten() {
                if entry.path().is_file() {
                    files.push(entry.path());
                }
            }
        }

        Ok(files)
    }

    fn get_user_files(&self) -> io::Result<Vec<PathBuf>> {
        let home = dirs_next::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;

        let files = vec![
            home.join(".profile"),
            home.join(".bash_profile"),
            home.join(".bash_login"),
            home.join(".bashrc"),
            home.join(".zshrc"),
            home.join(".cshrc"),
            home.join(".login"),
        ];

        Ok(files)
    }

    fn scan_file(&self, path: &Path, requires_sudo: bool) -> io::Result<Vec<PathLocation>> {
        let mut results = Vec::new();

        if !path.exists() {
            return Ok(results);
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            if self.path_regex.is_match(&line) {
                results.push(PathLocation {
                    file: path.to_path_buf(),
                    line_number: line_num + 1,
                    content: line,
                    requires_sudo,
                });
            }
        }

        Ok(results)
    }
}

#[allow(dead_code)]
/// Format the results in a user-friendly way
pub fn format_results(locations: &[PathLocation]) -> String {
    let mut output = String::new();

    output.push_str("System-level files (requires sudo):\n");
    for loc in locations.iter().filter(|l| l.requires_sudo) {
        output.push_str(&format!(
            "{}:{} - {}\n",
            loc.file.display(),
            loc.line_number,
            loc.content.trim()
        ));
    }

    output.push_str("\nUser-level files:\n");
    for loc in locations.iter().filter(|l| !l.requires_sudo) {
        output.push_str(&format!(
            "{}:{} - {}\n",
            loc.file.display(),
            loc.line_number,
            loc.content.trim()
        ));
    }

    output
}

#[allow(dead_code)]
// Example usage
fn main() -> io::Result<()> {
    let scanner = PathScanner::new();
    let results = scanner.scan_all()?;
    println!("{}", format_results(&results));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_path_scanner() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        let test_file = temp_dir.path().join("test.sh");

        let test_content = r#"
#!/bin/bash
export PATH="/usr/local/bin:$PATH"
path+=('/home/user/bin')
"#;

        // Print the exact content being tested
        println!("Test file content:");
        println!("-------------------");
        println!("{}", test_content);
        println!("-------------------");

        let mut file = File::create(&test_file)?;
        file.write_all(test_content.as_bytes())?;

        let scanner = PathScanner::new();
        let results = scanner.scan_file(&test_file, false)?;

        println!("\nMatches found: {}", results.len());
        for (i, result) in results.iter().enumerate() {
            println!("Match {}:", i + 1);
            println!("  Line number: {}", result.line_number);
            println!("  Content: {}", result.content.trim());
            println!("  File: {}", result.file.display());
        }

        println!("\nRegex pattern: {}", scanner.path_regex.as_str());

        assert_eq!(
            results.len(),
            2,
            "Expected exactly two PATH modifications, but found {}:\n{:#?}",
            results.len(),
            results
        );

        Ok(())
    }
}
