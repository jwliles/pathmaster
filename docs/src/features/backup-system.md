---
date_created: 2025-10-02T11-25-26
date_updated: 2025-08-30T02-49-20
timestamp: 1759404326262
title: backup-system
id: 4b2ba497-8c3e-4e5c-aff6-faa16617bac5
hash: 57b29c68a94efcdf53dda11e1081d166d699e9e1779efc51050d537263c53a1c
---
# Backup System

The pathmaster backup system provides comprehensive PATH and shell configuration backup capabilities with flexible modes and storage options.

## Backup Modes

### Default Mode
- Backs up both PATH and shell configurations
- Creates timestamped backups
- Maintains shell configuration integrity
- Recommended for most operations

### PATH-only Mode
- Backs up only PATH environment entries
- Faster operation for PATH-specific changes
- Useful for temporary modifications
- Minimal disk usage

### Shell-only Mode
- Backs up shell configuration files
- Preserves shell customizations
- Protects against configuration errors
- Essential for shell-specific changes

## Mode Selection

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

## Backup Storage

### Location
- Default: `~/.pathmaster/backups/`
- Organized by timestamp
- Separate directories for PATH and shell backups
- Clear naming convention for easy identification

### Format
- PATH backups stored in JSON format
- Shell configurations stored as .bak files
- Timestamped filenames
- Maintain original permissions

## Backup Management

### Creating Backups
- Automatic backup before modifications
- Manual backup creation available
- Configurable backup frequency
- Detailed backup logging

### Viewing History
```bash
# Show available backups
pathmaster history

# Lists:
# - Backup timestamps
# - Backup types
# - Storage locations
```

### Restoration
```bash
# Restore latest backup
pathmaster restore

# Restore specific backup
pathmaster restore --timestamp 20241218120000
```

## Best Practices

### Regular Backups
1. Create backups before major changes
2. Use appropriate backup modes
3. Verify backup creation
4. Test restoration process

### Backup Management
1. Review backup history regularly
2. Clean old backups periodically
3. Verify backup integrity
4. Document major changes
