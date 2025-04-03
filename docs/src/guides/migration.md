# Migration Guide

This guide helps you migrate from manual PATH management or other tools to pathmaster, and between different versions of pathmaster.

## Migrating from Manual PATH Management

If you've been managing your PATH manually in shell configuration files, follow these steps to migrate to pathmaster:

1. **Make a backup** of your current shell configuration:

```bash
cp ~/.bashrc ~/.bashrc.manual-backup
# or for other shells
cp ~/.zshrc ~/.zshrc.manual-backup
```

2. **Review your current PATH** to understand what's in it:

```bash
echo $PATH | tr ':' '\n'
```

3. **Add your custom directories** using pathmaster:

```bash
# Add each custom directory you want to keep
pathmaster add ~/bin ~/scripts /opt/custom/bin
```

4. **Check your PATH** to verify everything looks correct:

```bash
pathmaster list
```

5. **Remove any invalid entries**:

```bash
pathmaster flush
```

Pathmaster will automatically update your shell configuration file, commenting out or removing your previous PATH declarations and adding its own standardized format.

## Migrating from Other PATH Management Tools

If you're migrating from another PATH management tool:

1. **Disable or uninstall** the other tool first
2. **Clean up your shell configuration** to remove any entries it added
3. **Follow the steps above** to migrate from manual management

## Version Migration Notes

### Upgrading to 0.2.5

Version 0.2.5 includes:
- Enhanced documentation
- Improved error messages
- Better shell detection
- Expanded troubleshooting

No special migration steps are needed.

### Upgrading from 0.2.1 or earlier to 0.2.3+

Version 0.2.3 introduced configurable backup modes. After upgrading:

1. **Review the new backup options**:

```bash
pathmaster --help
```

2. **Choose your preferred backup mode** (if you want to change from the default):

```bash
# Example: Switch to backing up only PATH entries
pathmaster --backup-mode path
```

### Upgrading from 0.1.x to 0.2.x

The 0.2.x series introduced significant changes to PATH validation and backup systems:

1. **Check for invalid entries** with the improved validation:

```bash
pathmaster check
```

2. **Review your backups** to ensure they're working properly:

```bash
pathmaster history
```

3. **Test a restoration** to verify backup functionality:

```bash
# First make a current backup
pathmaster add . # This creates a backup even if no change is made
# Then try restoring it
pathmaster history # Note the most recent timestamp
pathmaster restore --timestamp <timestamp>
```

## Handling Breaking Changes

If you encounter issues after upgrading:

1. **Restore from backup**:

```bash
pathmaster restore
```

2. **Check the documentation** for changed behavior

3. **Report issues** on GitHub if you believe you've found a bug:
   https://github.com/jwliles/pathmaster/issues

## Best Practices for Future Migrations

1. **Always check release notes** before upgrading
2. **Create manual backups** of shell configuration files before major upgrades
3. **Test in a non-critical environment** if possible
4. **Keep note of any custom PATH entries** you've added manually
