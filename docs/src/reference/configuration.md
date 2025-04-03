# Configuration Options

Pathmaster provides several configuration options to customize its behavior. This page documents all available options and how to use them.

## Command-Line Options

### Global Options

These options can be used with any command:

| Option | Description |
|--------|-------------|
| `--help` | Display help information about pathmaster and its commands |
| `--version` | Display version information |
| `--backup-mode MODE` | Control what gets backed up when modifying PATH |

### Backup Mode Options

The `--backup-mode` flag accepts the following values:

| Mode | Description |
|------|-------------|
| `default` | Back up both PATH and shell configuration (default) |
| `path` | Back up only PATH entries |
| `shell` | Back up only shell configuration |
| `switch` | Toggle between PATH-only and shell-only backups |

Example usage:

```bash
# Back up only PATH when adding a directory
pathmaster --backup-mode path add ~/bin

# Back up only shell config when flushing invalid paths
pathmaster --backup-mode shell flush

# Reset to backing up both
pathmaster --backup-mode default add ~/other/bin

# Toggle between modes
pathmaster --backup-mode switch
```

## Command-Specific Options

### Restore Command

| Option | Description |
|--------|-------------|
| `--timestamp`, `-t` | Timestamp of the backup to restore |

Example usage:

```bash
# Restore from a specific backup
pathmaster restore --timestamp 20250401120000
```

## Environment Variables

Pathmaster respects the following environment variables:

| Variable | Description |
|----------|-------------|
| `PATH` | The main environment variable being managed |
| `SHELL` | Used to identify the appropriate configuration file |
| `HOME` | Used for expanding tildes (~) and locating config files |

## Configuration Files

Pathmaster does not use a dedicated configuration file, but it does interact with the following files:

| File | Purpose |
|------|---------|
| Shell configuration files | Modified to make PATH changes persistent |
| `~/.pathmaster/backups/` | Directory where backups are stored |

## Planned Future Options

In future versions, the following configuration options are planned:

- Custom backup location
- Configurable backup retention policy
- Integration with system package managers
- Custom path validation rules

## Best Practices

1. **Use Consistent Backup Modes**: Choose a backup mode that fits your workflow and stick with it
2. **Keep Backups Organized**: Periodically clean old backups you no longer need
3. **Use Version Control**: For critical systems, consider keeping shell config files in version control
4. **Document Custom Setups**: If you have a complex PATH setup, document your customizations

## Default Behaviors

When no explicit configuration is provided, pathmaster follows these defaults:

- Creates backups of both PATH and shell configurations before modifications
- Stores backups in `~/.pathmaster/backups/`
- Detects shell type from `$SHELL` environment variable
- Validates directories before adding them to PATH
- Keeps PATH entries unique (no duplicates)
- Expands tilde (`~`) to the user's home directory automatically
