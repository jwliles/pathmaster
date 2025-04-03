# Validation Commands

## Path Checking

### check Command

```bash
pathmaster check
```

### Validation Checks

- Directory existence
- Read permissions
- Path syntax
- Duplicate entries
- Invalid characters

### Output Format

```markdown
Validating PATH entries:
✓ /usr/local/bin
✓ /usr/bin
✗ /non/existent/path
✓ /bin
```

## Path Cleanup

### flush Command

```bash
pathmaster flush
```

### Features

- Removes invalid entries
- Creates backup first
- Updates shell config
- Shows removal summary
- Maintains valid paths

### Process

1. Backup creation
2. Path validation
3. Invalid path removal
4. Shell config update
5. Status report

### Example Output

```bash
Creating backup... Done
Checking PATH entries...
Removing invalid path: /non/existent/path
Removing invalid path: /old/tool/bin
Updated shell configuration
Removed 2 invalid entries
```

## Common Use Cases

### System Maintenance

```bash
# Regular PATH cleanup
pathmaster check
pathmaster flush
```

### Installation Cleanup

```bash
# After software removal
pathmaster check
pathmaster flush
```

### Path Verification

```bash
# Before adding new paths
pathmaster check
```

## Best Practices

### Regular Validation

1. Schedule regular checks
2. Review invalid paths
3. Document removals
4. Keep backups

### Cleanup Process

1. Check current PATH
2. Review invalid entries
3. Backup if needed
4. Run flush command
5. Verify changes

### Error Handling

1. Check error messages
2. Verify permissions
3. Review shell config
4. Test PATH updates
