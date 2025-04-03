# Troubleshooting

This guide helps you diagnose and resolve common issues you might encounter while using pathmaster.

## Common Issues and Solutions

### PATH Changes Not Persisting

**Symptoms**: You add or remove paths with pathmaster, but changes disappear after closing and reopening your terminal.

**Possible causes**:
1. Shell configuration file not being loaded
2. Permission issues with config file
3. Configuration conflicts with other tools

**Solutions**:
1. **Check shell startup files**: Ensure your shell loads the configuration file that pathmaster modifies
   ```bash
   # For bash, check if .bashrc is sourced from .bash_profile or .profile
   cat ~/.bash_profile
   ```

2. **Check file permissions**:
   ```bash
   ls -la ~/.bashrc  # or your shell's config file
   ```

3. **Manually verify changes**:
   ```bash
   # Look for pathmaster entries in your config
   grep -A 2 "pathmaster" ~/.bashrc
   ```

4. **Force reload your shell configuration**:
   ```bash
   source ~/.bashrc  # or your shell's config file
   ```

### Invalid PATH Entries Remain

**Symptoms**: You run `pathmaster flush` but invalid directories still appear in your PATH.

**Possible causes**:
1. PATH is being set elsewhere in your configuration
2. Something is overriding pathmaster's changes
3. Shell caching issues

**Solutions**:
1. **Check for other PATH modifications**:
   ```bash
   grep -r "PATH=" ~/.bash*
   ```

2. **Clear invalid paths and explicitly set PATH**:
   ```bash
   pathmaster flush
   source ~/.bashrc  # or your shell's config file
   ```

3. **Verify current PATH**:
   ```bash
   pathmaster list
   echo $PATH | tr ':' '\n'  # Compare with raw PATH
   ```

### "Command Not Found" After Using pathmaster

**Symptoms**: After modifying PATH, you get "command not found" errors for commands that worked before.

**Possible causes**:
1. Essential directories accidentally removed
2. PATH order changed, affecting which version of a command is found first
3. Shell hasn't reloaded new PATH

**Solutions**:
1. **Restore from backup**:
   ```bash
   pathmaster restore
   ```

2. **Check current PATH**:
   ```bash
   pathmaster list
   ```

3. **Add back essential directories**:
   ```bash
   pathmaster add /usr/bin /bin /usr/sbin /sbin
   ```

4. **Reload shell**:
   ```bash
   exec $SHELL -l
   ```

### Backup and Restore Issues

**Symptoms**: Backups aren't being created, or restoration fails.

**Possible causes**:
1. Permission issues with backup directory
2. Backup file corruption
3. No backups available

**Solutions**:
1. **Check backup directory**:
   ```bash
   ls -la ~/.pathmaster/backups/
   ```

2. **Create the backup directory if missing**:
   ```bash
   mkdir -p ~/.pathmaster/backups/
   ```

3. **List available backups**:
   ```bash
   pathmaster history
   ```

4. **Force a new backup**:
   ```bash
   # This creates a backup even without changes
   pathmaster add .
   pathmaster delete .
   ```

### Shell Detection Issues

**Symptoms**: Pathmaster doesn't correctly detect your shell.

**Possible causes**:
1. SHELL environment variable not set correctly
2. Using an unsupported shell
3. Custom shell configuration

**Solutions**:
1. **Check your SHELL variable**:
   ```bash
   echo $SHELL
   ```

2. **Try setting SHELL explicitly**:
   ```bash
   SHELL=/bin/bash pathmaster add ~/bin
   ```

3. **Manually update your shell config**:
   Add pathmaster-managed directories to your PATH manually.

## Diagnostic Commands

Use these commands to diagnose issues:

1. **Check pathmaster version**:
   ```bash
   pathmaster --version
   ```

2. **Validate your PATH**:
   ```bash
   pathmaster check
   ```

3. **List backup history**:
   ```bash
   pathmaster history
   ```

4. **View current PATH**:
   ```bash
   pathmaster list
   ```

5. **Examine your shell environment**:
   ```bash
   env | grep PATH
   echo $SHELL
   ```

## When All Else Fails

If you encounter persistent issues:

1. **Restore from backup**:
   ```bash
   pathmaster restore
   ```

2. **Check the GitHub repository** for known issues:
   https://github.com/jwliles/pathmaster/issues

3. **Report a new issue** with detailed information:
   - Your shell and OS version
   - Steps to reproduce the problem
   - Error messages you received
   - Output of `pathmaster list` and `pathmaster check`

4. **Manually reset PATH** in your shell configuration file if needed
