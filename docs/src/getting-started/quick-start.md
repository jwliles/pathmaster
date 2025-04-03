# Quick Start Guide

## Basic Operations

### Adding Directories

```bash
# Add a single directory
pathmaster add ~/bin

# Add multiple directories
pathmaster add ~/bin ~/scripts /usr/local/bin
```

### Viewing Current PATH

```bash
# List all PATH entries
pathmaster list
```

### Removing Directories

```bash
# Remove a directory
pathmaster delete ~/old/bin

# Remove multiple directories
pathmaster delete ~/old/bin ~/deprecated/scripts
```

## Path Validation

### Check for Invalid Paths

```bash
# Validate current PATH
pathmaster check
```

### Remove Invalid Paths

```bash
# Clean up invalid entries
pathmaster flush
```

## Backup Management

### Configuring Backup Mode

```bash
# Default (both PATH and shell)
pathmaster --backup-mode default

# PATH-only backup
pathmaster --backup-mode path

# Shell-only backup
pathmaster --backup-mode shell

# Toggle between modes
pathmaster --backup-mode switch
```

### Managing Backups

```bash
# View backup history
pathmaster history

# Restore from latest backup
pathmaster restore

# Restore specific backup
pathmaster restore --timestamp 20240301120000
```

## Common Workflows

### Adding New Development Tools

1. Create development bin directory:

```bash
mkdir -p ~/dev/bin
```

2. Add to PATH:

```bash
pathmaster add ~/dev/bin
```

3. Verify addition:

```bash
pathmaster list
```

### Cleaning Up PATH

1. Check for invalid entries:

```bash
pathmaster check
```

2. Remove invalid entries:

```bash
pathmaster flush
```

3. Verify changes:

```bash
pathmaster list
```

### Backup and Restore

1. Configure backup mode:

```bash
pathmaster --backup-mode default
```

2. Make changes to PATH
3. View backup history:

```bash
pathmaster history
```

4. Restore if needed:

```bash
pathmaster restore
```

## Next Steps

- Read the [Basic Usage Guide](basic-usage.md) for more details
- Check the [Command Reference](../commands/overview.md) for all available commands
- Review [Features](../features/overview.md) for advanced functionality
