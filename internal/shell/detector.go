package shell

import (
	"os"
	"path/filepath"
	"strings"
)

// ShellType represents the type of shell
type ShellType string

const (
	ShellUnknown ShellType = "unknown"
	ShellBash    ShellType = "bash"
	ShellZsh     ShellType = "zsh"
	ShellFish    ShellType = "fish"
	ShellKsh     ShellType = "ksh"
	ShellTcsh    ShellType = "tcsh"
)

// DetectCurrentShell attempts to determine the current shell
func DetectCurrentShell() ShellType {
	// Try using SHELL environment variable first
	shellPath := os.Getenv("SHELL")
	if shellPath != "" {
		return detectShellFromPath(shellPath)
	}
	
	// Fallback to parent process name
	// Not implemented in this basic version
	
	return ShellUnknown
}

// detectShellFromPath determines shell type from path
func detectShellFromPath(path string) ShellType {
	shell := filepath.Base(path)
	
	switch {
	case strings.Contains(shell, "bash"):
		return ShellBash
	case strings.Contains(shell, "zsh"):
		return ShellZsh
	case strings.Contains(shell, "fish"):
		return ShellFish
	case strings.Contains(shell, "ksh"):
		return ShellKsh
	case strings.Contains(shell, "tcsh"):
		return ShellTcsh
	default:
		return ShellUnknown
	}
}