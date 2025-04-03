# Error Handling

Pathmaster is designed to handle errors gracefully and provide clear, helpful error messages. This guide explains common errors you might encounter and how to resolve them.

## Common Error Types

### Permission Errors

```
Error: Permission denied when modifying /etc/bin
```

**Cause**: You're trying to add a directory that requires elevated permissions.

**Solution**: 
- For system directories, use sudo: `sudo pathmaster add /opt/special/bin`
- Add user-level directories instead (~/.local/bin) when possible

### Invalid Path Errors

```
Error: Directory does not exist: /home/user/nonexistent
```

**Cause**: You're trying to add a directory that doesn't exist.

**Solution**:
- Create the directory first: `mkdir -p /path/to/directory`
- Check for typos in the path
- Use tab-completion to avoid path errors

### Shell Configuration Errors

```
Error: Could not update shell configuration file ~/.bashrc
```

**Cause**: Pathmaster couldn't modify your shell configuration file.

**Solution**:
- Check file permissions: `ls -la ~/.bashrc`
- Ensure the file exists: `touch ~/.bashrc`
- Manually update your PATH if needed

### Backup Related Errors

```
Error: Could not create backup: Permission denied
```

**Cause**: Pathmaster couldn't create a backup before making changes.

**Solution**:
- Check permissions on the backup directory: `ls -la ~/.pathmaster`
- Create the directory if missing: `mkdir -p ~/.pathmaster/backups`

### Restore Errors

```
Error: No backup found with timestamp 20250401120000
```

**Cause**: You're trying to restore from a backup that doesn't exist.

**Solution**:
- List available backups: `pathmaster history`
- Choose an existing timestamp
- Use the most recent backup if unsure: `pathmaster restore`

## How Pathmaster Handles Errors

Pathmaster follows these principles for error handling:

1. **Safety First**: Pathmaster creates backups before making changes
2. **Validation**: Directories are validated before being added to PATH
3. **Rollback**: If an error occurs during an operation, pathmaster tries to roll back changes
4. **Clear Messages**: Error messages include specific details about what went wrong
5. **Exit Codes**: Pathmaster returns non-zero exit codes on error for script integration

## Debugging Techniques

When encountering persistent issues:

1. **Check PATH**: Run `pathmaster list` to see current PATH entries
2. **Check Invalid Entries**: Run `pathmaster check` to identify invalid directories
3. **Review Backups**: Run `pathmaster history` to see backup history
4. **Restore if Needed**: Run `pathmaster restore` to revert to a known good state
5. **Check Shell Config**: Examine your shell configuration file for issues

## Error Exit Codes

Pathmaster uses the following exit codes:

- **0**: Success
- **1**: General error
- **2**: Invalid input or usage
- **3**: Permission denied
- **4**: Resource not found

## Best Practices

- Always have a backup strategy beyond pathmaster's automatic backups
- Check your PATH occasionally with `pathmaster check`
- Flush invalid paths periodically with `pathmaster flush`
- Keep a simple, clean PATH to avoid conflicts
