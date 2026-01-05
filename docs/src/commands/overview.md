---
date_created: 2025-10-02T11-25-26
date_updated: 2025-08-30T02-49-20
timestamp: 1759404326261
title: overview
id: bde5947e-a6e7-4556-8df1-b72b6323a2ff
hash: 0e85c66021f805c2f09df1b94e09c3a53732329a1a444896dfa7efe2dafeb1bf
---
# Command Overview

## Directory Management Commands

### add

Add directories to PATH

```bash
pathmaster add <directory>...

Options:
  <directory>...    One or more directories to add

Examples:
  pathmaster add ~/bin
  pathmaster add ~/bin ~/scripts /usr/local/bin
```

### delete

Remove directories from PATH

```bash
pathmaster delete <directory>...

Options:
  <directory>...    One or more directories to remove

Aliases:
  remove            Alternative to delete command

Examples:
  pathmaster delete ~/old/bin
  pathmaster remove ~/deprecated/scripts
```

### list

Display current PATH entries

```bash
pathmaster list

Output Format:
  - One entry per line
  - Full path displayed
  - Invalid paths marked
```

## Validation Commands

### check

Validate PATH entries

```bash
pathmaster check

Checks:
  - Directory existence
  - Read permissions
  - Invalid characters
  - Duplicate entries
```

### flush

Remove invalid entries

```bash
pathmaster flush

Actions:
  1. Creates backup
  2. Identifies invalid paths
  3. Removes invalid entries
  4. Updates shell config
  5. Provides removal summary
```

## Backup Commands

### --backup-mode

Configure backup behavior

```bash
pathmaster --backup-mode <mode>

Modes:
  default    Back up both PATH and shell config
  path       Back up only PATH entries
  shell      Back up only shell config
  switch     Toggle between modes

Example:
  pathmaster --backup-mode path add ~/bin
```

### history

Show backup history

```bash
pathmaster history

Output:
  - Backup timestamps
  - Backup file locations
  - Backup types (PATH/shell/both)
```

### restore

Restore from backup

```bash
pathmaster restore [--timestamp <time>]

Options:
  --timestamp    Specific backup to restore (YYYYMMDDHHMMSS)

Examples:
  pathmaster restore
  pathmaster restore --timestamp 20240301120000
```

## Global Options

### --help

Display help information

```bash
pathmaster --help
pathmaster <command> --help
```

### --version

Show version information

```bash
pathmaster --version
```

## Command Return Values

All commands return:

- 0 for success
- 1 for general errors
- 2 for invalid arguments

## Command Combinations

Commands can be combined with backup modes:

```bash
# Add with PATH-only backup
pathmaster --backup-mode path add ~/bin

# Flush with shell-only backup
pathmaster --backup-mode shell flush
```
