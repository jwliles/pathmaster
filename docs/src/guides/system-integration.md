# System Integration

This guide explains how to integrate pathmaster with other tools, scripts, and system components for a seamless experience.

## Integrating with Shell Startup

Pathmaster automatically integrates with your shell configuration files, but you can enhance this integration:

### Adding to Login Scripts

For system-wide PATH initialization, you can call pathmaster in your login scripts:

```bash
# In /etc/profile or similar
if command -v pathmaster >/dev/null 2>&1; then
    # Run a validation check at login
    pathmaster check >/dev/null 2>&1
fi
```

### Shell Aliases

Add these aliases to your shell configuration for quicker access:

```bash
# Bash/Zsh
alias pm='pathmaster'
alias pmlist='pathmaster list'
alias pmcheck='pathmaster check'
alias pmflush='pathmaster flush'

# Fish
alias pm 'pathmaster'
alias pmlist 'pathmaster list'
alias pmcheck 'pathmaster check'
alias pmflush 'pathmaster flush'
```

## Script Integration

Pathmaster can be integrated into automation scripts:

### Exit Code Handling

Pathmaster returns standard exit codes that you can use in scripts:

```bash
#!/bin/bash
# Example script using pathmaster

# Add a directory to PATH
pathmaster add ~/new-scripts

# Check the exit code
if [ $? -eq 0 ]; then
    echo "Directory added successfully"
else
    echo "Failed to add directory"
    exit 1
fi

# Check for invalid paths
pathmaster check
if [ $? -ne 0 ]; then
    echo "Warning: Invalid paths detected"
fi
```

### Capturing Output

You can capture and parse pathmaster output in scripts:

```bash
#!/bin/bash
# Get current PATH as a list
paths=$(pathmaster list | grep -v "Current PATH")

# Process each path
echo "$paths" | while read -r line; do
    echo "Processing: $line"
    # Do something with each path...
done
```

## System Maintenance Integration

### Cron Jobs

You can set up periodic maintenance with cron:

```bash
# Add this to crontab -e
# Check and flush invalid paths weekly
0 0 * * 0 /usr/bin/pathmaster flush >/dev/null 2>&1
```

### System Updates

Add pathmaster checks to post-update scripts:

```bash
#!/bin/bash
# After system update script

# Update system packages
apt-get update && apt-get upgrade -y

# Check PATH for any broken links after update
pathmaster check

# Optionally flush invalid paths
# pathmaster flush
```

## Package Manager Integration

If you're building packages that need to add directories to PATH:

### Post-installation Hook

```bash
#!/bin/bash
# Post-install script

# If pathmaster is available, use it
if command -v pathmaster >/dev/null 2>&1; then
    pathmaster add /opt/my-package/bin
else
    # Fallback: Update PATH in standard shell configs
    echo 'export PATH="$PATH:/opt/my-package/bin"' >> /etc/profile.d/my-package.sh
fi
```

## Directory Service Integration

For enterprise environments with managed directories:

```bash
#!/bin/bash
# Directory service integration script

# Get list of valid application paths from directory service
valid_paths=$(ldapsearch -x -LLL "(&(objectClass=applicationPath)(status=active))" path | grep "^path:" | awk '{print $2}')

# Add each valid path
echo "$valid_paths" | while read -r path; do
    if [ -d "$path" ]; then
        pathmaster add "$path"
    fi
done
```

## Containerization

For Docker or containerized environments:

```dockerfile
# Example Dockerfile with pathmaster
FROM ubuntu:latest

# Install pathmaster
RUN apt-get update && apt-get install -y cargo
RUN cargo install pathmaster

# Setup container paths
RUN pathmaster add /app/bin /custom/tools

# ...rest of Dockerfile
```

## Best Practices

1. **Check before modification**: In scripts, use `pathmaster check` before making changes
2. **Handle errors gracefully**: Always check exit codes in scripts
3. **Backup when automating**: Consider creating manual backups before automated changes
4. **Limit automation scope**: Be cautious with automation that might modify PATH
5. **Test integrations**: Verify your integrations in a safe environment first
