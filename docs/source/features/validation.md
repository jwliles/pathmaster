# Validation

pathmaster includes comprehensive validation functionality to ensure PATH integrity and prevent common issues.

## Path Validation

### Directory Validation
- Existence checking
- Permission verification
- Path normalization
- Symbolic link resolution

### Syntax Validation
- Path format checking
- Character validation
- Length constraints
- Shell compatibility

### Access Validation
- Read permissions
- Execute permissions
- Owner verification
- Group permissions

## Validation Commands

### Check Command
```bash
# Validate current PATH
pathmaster check

# Output shows:
# ✓ Valid directories
# ✗ Invalid directories
# ! Permission issues
```

### Flush Command
```bash
# Remove invalid entries
pathmaster flush

# Actions:
# 1. Creates backup
# 2. Validates entries
# 3. Removes invalid paths
# 4. Updates configuration
```

## Validation Types

### Basic Validation
- Directory existence
- Basic permissions
- Path formatting
- Duplicate detection

### Enhanced Validation
- Recursive permission checking
- Symbolic link validation
- Ownership verification
- File system access

### Shell-Specific Validation
- Configuration syntax
- Export statements
- Path arrays
- Framework compatibility

## Error Handling

### Common Issues
- Missing directories
- Permission denied
- Invalid characters
- Broken links

### Error Messages
- Clear descriptions
- Resolution suggestions
- Relevant commands
- Documentation references

## Best Practices

### Regular Validation
1. Schedule periodic checks
2. Review invalid entries
3. Document removals
4. Maintain backups

### Validation Process
1. Check current PATH
2. Review invalid entries
3. Backup if needed
4. Remove invalid entries
5. Verify changes

### Troubleshooting
1. Check error messages
2. Verify permissions
3. Review shell config
4. Test PATH updates