# Shell Configuration

Pathmaster interacts with your shell configuration files to make PATH changes persistent across sessions. This page details how pathmaster works with different shell environments.

## Supported Shells

Pathmaster automatically detects and supports the following shells:

| Shell    | Configuration File                | Detection Method       |
|----------|----------------------------------|------------------------|
| Bash     | `~/.bashrc`                      | `$SHELL` contains "bash"|
| Zsh      | `~/.zshrc`                       | `$SHELL` contains "zsh" |
| Fish     | `~/.config/fish/config.fish`     | `$SHELL` contains "fish"|
| Tcsh/Csh | `~/.tcshrc`                      | `$SHELL` contains "tcsh" or "csh"|
| Ksh      | `~/.kshrc`                       | `$SHELL` contains "ksh" |

If your shell isn't detected, a generic handler is used as a fallback.

## How Shell Detection Works

When you run pathmaster, it:

1. Reads the `$SHELL` environment variable
2. Extracts the shell name by checking for keywords in the path
3. Selects the appropriate handler for your shell type
4. Locates your shell's configuration file

## Modification Approach

When modifying your PATH, pathmaster:

1. Creates a backup of your shell configuration file (with `.bak` extension and timestamp)
2. Scans for existing PATH-related statements in your configuration
3. Removes any pathmaster-managed PATH entries
4. Adds standardized statements with all required paths
5. Adds a timestamp comment to indicate when changes were made

## Shell-Specific Implementations

### Bash

```bash
# Added by pathmaster on 2025-04-02 15:04:32
export PATH="/usr/local/bin:/usr/bin:/bin:/home/user/bin"
```

### Zsh

```bash
# Added by pathmaster on 2025-04-02 15:04:32
path=(/usr/local/bin /usr/bin /bin /home/user/bin) && export PATH
```

### Fish

```fish
# Added by pathmaster on 2025-04-02 15:04:32
set -e PATH
fish_add_path /usr/local/bin
fish_add_path /usr/bin
fish_add_path /bin
fish_add_path /home/user/bin
```

### Tcsh

```tcsh
# Added by pathmaster on 2025-04-02 15:04:32
setenv PATH /usr/local/bin:/usr/bin:/bin:/home/user/bin
```

### Ksh

```bash
# Added by pathmaster on 2025-04-02 15:04:32
export PATH=/usr/local/bin:/usr/bin:/bin:/home/user/bin
```

## Best Practices

1. **Let pathmaster manage your PATH**: Avoid manually editing pathmaster-managed PATH statements
2. **Check after changes**: Use `pathmaster list` to verify your PATH looks correct
3. **Use restore if needed**: If shell configuration gets corrupted, use `pathmaster restore` to revert changes
4. **Keep configuration simple**: Avoid complex PATH manipulations that might conflict with pathmaster

## Troubleshooting

- If changes aren't persisting, ensure your shell is loading the configuration file pathmaster modifies
- If you get "Permission denied" errors, check file permissions on your shell configuration
- If you use multiple shell profiles, pathmaster only modifies the main configuration file
- For custom shells or configurations, you may need to manually update your PATH
