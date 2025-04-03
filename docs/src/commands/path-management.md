# PATH Management Commands

## Directory Addition

### Basic Usage

```bash
pathmaster add <directory>...
```

### Features

- Validates directories before adding
- Expands path variables and ~
- Prevents duplicate entries
- Creates automatic backups
- Updates shell configuration

### Examples

```bash
# Add single directory
pathmaster add ~/bin

# Add multiple directories
pathmaster add ~/bin ~/scripts /usr/local/bin

# Add development tools
pathmaster add /opt/toolchain/bin
```

## Directory Removal

### Basic Usage

```bash
pathmaster delete <directory>...
```

### Features

- Safe removal with backups
- Handles multiple directories
- Updates shell configuration
- Maintains PATH order
- Ignores non-existent paths

### Examples

```bash
# Remove single directory
pathmaster delete ~/old/bin

# Remove multiple directories
pathmaster delete ~/old/bin ~/deprecated/scripts

# Remove using full path
pathmaster delete /opt/old-version/bin
```

## PATH Listing

### Basic Usage

```bash
pathmaster list
```

### Output Format

```text
Current PATH entries:
- /usr/local/bin
- ~/bin
- /usr/bin
- /bin
```

### Features

- Shows full expanded paths
- Indicates invalid entries
- Maintains order
- Clear formatting

## Best Practices

### Adding Directories

1. Verify directory exists
2. Check permissions
3. Use absolute paths
4. Verify changes

### Removing Directories

1. List current PATH first
2. Create backup
3. Remove directories
4. Verify removal

### Path Management

1. Regular cleanup
2. Document changes
3. Test new paths
4. Keep backups
