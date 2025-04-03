# Backup Commands

## Backup Mode Selection

The backup system supports different modes of operation through the `--backup-mode` flag.

```bash
pathmaster --backup-mode <mode>
```

### Available Modes

#### default

```bash
pathmaster --backup-mode default
```

- Backs up both PATH and shell configuration
- Most comprehensive backup option
- Recommended for major changes

#### path

```bash
pathmaster --backup-mode path
```

- Backs up only PATH entries
- Faster operation
- Suitable for temporary PATH changes

#### shell

```bash
pathmaster --backup-mode shell
```

- Backs up only shell configuration
- Preserves shell customizations
- Useful for shell-specific changes

#### switch

```bash
pathmaster --backup-mode switch
```

- Toggles between backup modes
- Cycles: PATH → SHELL → DEFAULT
- Quick mode switching

## Backup History

View available backups using:

```bash
pathmaster history
```

Output shows:

- Backup timestamps
- Backup types
- File locations

## Restore Operations

### Latest Backup

```bash
pathmaster restore
```

- Restores most recent backup
- Automatic mode detection
- Updates current session

### Specific Backup

```bash
pathmaster restore --timestamp 20240301120000
```

- Restores specific backup point
- Requires exact timestamp
- Full system state recovery

## Best Practices

### Regular Backups

- Create backups before major changes
- Use appropriate backup modes
- Verify backup creation
- Test restoration process

### Backup Management

- Review backup history regularly
- Clean old backups periodically
- Verify backup integrity
- Document major changes
