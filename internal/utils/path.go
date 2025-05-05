package utils

import (
	"os"
	"path/filepath"
	"strings"
)

// GetPathEntries returns all directories in the PATH as a slice
func GetPathEntries() []string {
	pathVar := os.Getenv("PATH")
	if pathVar == "" {
		return []string{}
	}
	
	return strings.Split(pathVar, string(os.PathListSeparator))
}

// ExpandPath expands a path with tilde and env vars
func ExpandPath(path string) string {
	// Expand tilde to home directory
	if strings.HasPrefix(path, "~/") {
		home, err := os.UserHomeDir()
		if err == nil {
			path = filepath.Join(home, path[2:])
		}
	}
	
	// Expand environment variables
	return os.ExpandEnv(path)
}

// PathExists checks if a path exists
func PathExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

// IsPathValid checks if a path exists and is a directory
func IsPathValid(path string) bool {
	info, err := os.Stat(path)
	if err != nil {
		return false
	}
	return info.IsDir()
}