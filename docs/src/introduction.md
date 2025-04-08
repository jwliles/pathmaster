# Introduction to pathmaster

Pathmaster is a command-line tool for managing your system's PATH environment variable. It provides safe, efficient PATH manipulation with automatic backups and comprehensive validation.

## What is PATH?

The PATH environment variable is a fundamental part of Unix-like operating systems that tells your system where to look for executable programs. When you type a command like `python` or `git`, your system searches through the directories listed in PATH to find these programs.

## Why Use pathmaster?

Managing PATH manually can be risky and error-prone. Pathmaster helps by:

- Safely adding and removing directories
- Automatically creating backups before changes
- Validating paths to prevent errors
- Managing shell configuration files
- Supporting multiple shell types

## Key Features

- **Safe Operations**: All changes are backed up automatically
- **Validation**: Checks for invalid or non-existent paths
- **Multi-Shell Support**: Works with bash, zsh, fish, tcsh, and ksh
- **Backup System**: Flexible backup modes and easy restoration
- **User-Friendly**: Clear feedback and error messages

## Version Information

Current version: 0.2.6

- Enhanced documentation
- Improved shell detection and configuration management
- Fixed duplicate path entries in zsh configurations
- Better error handling
- Comprehensive backup mode support
