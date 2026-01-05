---
date_created: 2025-10-02T11-25-26
date_updated: 2025-08-30T02-49-20
timestamp: 1759404326262
title: backup
id: e053708b-cde4-4c37-864c-f8bd793b8346
hash: 3bdfa635d453e7ddf622795ebb160d7d13539a80b8b8471d5526edf0805308a8
---
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
