package main

import (
	"fmt"
	"os"
	
	"github.com/jwliles/pathmaster/internal/commands"
)

const version = "0.1.0-dev"

func main() {
	// Create the root command
	rootCmd := commands.NewRootCommand(version)
	
	// Register subcommands (to be implemented)
	// rootCmd.RegisterCommand("add", &commands.AddCommand{})
	// rootCmd.RegisterCommand("list", &commands.ListCommand{})
	// rootCmd.RegisterCommand("check", &commands.CheckCommand{})
	// rootCmd.RegisterCommand("delete", &commands.DeleteCommand{})
	// rootCmd.RegisterCommand("flush", &commands.FlushCommand{})
	
	// Execute the command
	if err := rootCmd.Execute(os.Args[1:]); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %s\n", err)
		os.Exit(1)
	}
}