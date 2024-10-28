#[derive(Debug, Clone, PartialEq)]
pub enum ShellType {
    Zsh,
    Bash,
    Fish,
    Tcsh,
    Ksh,
    Generic,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModificationType {
    Assignment,        // export PATH=...
    Addition,          // PATH=$PATH:... or fish_add_path
    ArrayModification, // path=(...) in zsh
    SetEnv,            // setenv PATH ... in tcsh
    FishPath,          // set -gx PATH ... in fish
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PathModification {
    pub line_number: usize,
    pub content: String,
    pub modification_type: ModificationType,
}
