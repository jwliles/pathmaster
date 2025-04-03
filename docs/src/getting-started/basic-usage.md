# Basic Usage Guide

## Overview

This guide covers the essential operations you'll perform with pathmaster, including PATH management, validation, and backup handling.

## Managing PATH Entries

### Adding Directories

```bash
# Single directory
pathmaster add ~/bin

# Multiple directories
pathmaster add ~/bin ~/scripts /usr/local/bin
```

Key points:

- Directories are validated before adding
- Duplicates are automatically prevented
- Paths are normalized and expanded
- Shell config is automatically updated

### Removing Directories

```bash
# Single directory
pathmaster delete ~/old/bin

# Multiple directories
pathmaster delete ~/old/bin ~/deprecated/scripts
```

Key points:

- Safe removal with automatic backups
- Shell config is updated
- Current session PATH is updated
- Non-existent paths are ignored

### Viewing PATH

```bash
pathmaster list
```

Output shows:

- Current PATH entries
- One entry per line
- Full expanded paths
- Invalid paths marked

## Path Validation

### Checking Paths

```bash
pathmaster check
```

Validates:

- Directory existence
- Read permissions
- Path syntax
- Duplicate entries

### Cleaning Invalid Paths

```bash
pathmaster flush
```

Process:

1. Creates backup
2. Checks all PATH entries
3. Removes invalid entries
4. Updates shell config
5. Shows removal summary

## Backup Management

### Configuring Backups

```bash
# Default mode (both PATH and shell)
pathmaster --backup-mode default

# PATH-only backups
pathmaster --backup-mode path

# Shell-only backups
pathmaster --backup-mode shell

# Toggle between modes
pathmaster --backup-mode switch
```

### Managing Backups

```bash
# View backup history
pathmaster history

# Restore latest backup
pathmaster restore

# Restore specific backup
pathmaster restore --timestamp 20240301120000
```

## Best Practices

### Regular Maintenance

1. Check PATH regularly:

```bash
pathmaster check
```

2. Clean invalid entries:

```bash
pathmaster flush
```

3. Verify changes:

```bash
pathmaster list
```

### Safe Changes

1. Configure backup mode:

```bash
pathmaster --backup-mode default
```

2. Make changes
3. Verify results
4. Keep backup history

### Troubleshooting

1. Check command output
2. Review error messages
3. Use backup history
4. Restore if needed

## Next Steps

- Explore advanced features
- Review command documentation
- Check backup history
- Set up regular maintenance
