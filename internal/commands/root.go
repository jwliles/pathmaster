package commands

import (
	"fmt"
	"os"
)

// CommandRunner defines the interface for command execution
type CommandRunner interface {
	Execute(args []string) error
}

// RootCommand is the main command that dispatches to subcommands
type RootCommand struct {
	subcommands map[string]CommandRunner
	version     string
}

// NewRootCommand creates a new root command
func NewRootCommand(version string) *RootCommand {
	return &RootCommand{
		subcommands: make(map[string]CommandRunner),
		version:     version,
	}
}

// RegisterCommand adds a subcommand to the root command
func (r *RootCommand) RegisterCommand(name string, cmd CommandRunner) {
	r.subcommands[name] = cmd
}

// Execute runs the command with the given arguments
func (r *RootCommand) Execute(args []string) error {
	if len(args) < 1 {
		return r.showHelp()
	}

	// Check for help flag
	if args[0] == "--help" || args[0] == "-h" {
		return r.showHelp()
	}

	// Check for version flag
	if args[0] == "--version" || args[0] == "-v" {
		fmt.Printf("pathmaster version %s\n", r.version)
		return nil
	}

	// Dispatch to subcommand
	if cmd, ok := r.subcommands[args[0]]; ok {
		return cmd.Execute(args[1:])
	}

	fmt.Fprintf(os.Stderr, "Unknown command: %s\n", args[0])
	return r.showHelp()
}

// showHelp displays the help message
func (r *RootCommand) showHelp() error {
	fmt.Println("Pathmaster - A tool for managing your system's PATH environment variable")
	fmt.Println("")
	fmt.Println("Usage:")
	fmt.Println("  pathmaster [command] [arguments]")
	fmt.Println("")
	fmt.Println("Available Commands:")
	
	// This would list all registered commands
	// For now, just show some placeholders
	fmt.Println("  add       Add a directory to PATH")
	fmt.Println("  list      List current PATH entries")
	fmt.Println("  check     Validate PATH entries")
	fmt.Println("  delete    Remove a directory from PATH")
	fmt.Println("  flush     Remove invalid entries from PATH")
	fmt.Println("  backup    Backup operations for PATH")
	
	fmt.Println("")
	fmt.Println("Flags:")
	fmt.Println("  -h, --help     Show help for command")
	fmt.Println("  -v, --version  Show version information")
	
	return nil
}