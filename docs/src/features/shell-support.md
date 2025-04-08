# Shell Support

pathmaster provides comprehensive support for various shell environments, ensuring consistent PATH management across different shells.

## Supported Shells

### bash
- Primary shell support
- .bashrc management
- PATH export handling
- Environment variable support

### zsh
- Full zsh compatibility
- .zshrc management
- path array support (both path=() and path+=() formats)
- Prevents duplicate PATH entries
- Framework compatibility

### fish
- Modern shell support
- config.fish handling
- fish_add_path support
- Universal variable handling

### tcsh/csh
- C shell compatibility
- .tcshrc management
- setenv PATH support
- Shell-specific syntax

### ksh
- Korn shell support
- .kshrc management
- typeset handling
- Shell initialization order

## Configuration Management

### File Detection
- Automatic shell detection
- Proper config file location
- Framework awareness
- Permission handling

### File Modifications
- Safe configuration updates
- Automatic backups
- Syntax preservation
- Prevent duplicate path entries
- Proper in-place updates
- Error handling

### Shell Integration
- Framework compatibility
- Plugin support
- Custom configuration
- Path management commands

## Shell-Specific Features

### bash/zsh Features
```bash
# PATH modification
export PATH="/new/path:$PATH"

# Array handling
path=(/usr/bin /usr/local/bin) && export PATH
```

### fish Features
```bash
# PATH modification
fish_add_path /new/path

# Universal variables
set -U fish_user_paths /new/path $fish_user_paths
```

### tcsh Features
```bash
# PATH modification
setenv PATH /new/path:$PATH

# Path array
set path = (/usr/bin /usr/local/bin)
```

## Framework Support

### Oh My Zsh
- Compatible with custom plugins
- Preserves theme configurations
- Handles custom PATH modifications
- Framework-specific backup support

### Oh My Fish
- Supports fish_add_path
- Maintains framework structure
- Plugin compatibility
- Custom function preservation

## Best Practices

### Configuration Updates
1. Use shell-appropriate commands
2. Verify configuration changes
3. Test PATH modifications
4. Maintain backups

### Framework Integration
1. Check framework compatibility
2. Use framework-specific methods
3. Test with custom configurations
4. Document special requirements
