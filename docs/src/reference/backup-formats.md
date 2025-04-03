# Backup Formats

Pathmaster creates and manages backups of your PATH environment variable to ensure you can recover from unintended changes. This page details the backup formats and storage mechanisms.

## Backup Storage Location

By default, pathmaster stores all backups in:

```
~/.pathmaster/backups/
```

This directory is created automatically when you first run a command that modifies your PATH.

## PATH Backup Format

PATH backups are stored as JSON files with the naming pattern:

```
backup_YYYYMMDDHHMMSS.json
```

Where `YYYYMMDDHHMMSS` is the timestamp when the backup was created.

### JSON Structure

Each backup file contains a simple JSON structure:

```json
{
  "timestamp": "20250402150432",
  "path": "/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:/home/user/bin"
}
```

The file includes:
- `timestamp`: When the backup was created (format: YYYYMMDDHHMMSS)
- `path`: The complete PATH string at the time of backup

## Shell Configuration Backups

When pathmaster modifies your shell configuration files, it first creates backup copies with the extension `.bak` and timestamp:

```
~/.bashrc.bak_20250402150432
```

These backups preserve your original shell configuration before any modifications.

## Backup Modes

Pathmaster supports different backup modes to control what gets backed up:

- **Default**: Both PATH and shell configurations are backed up
- **Path Only**: Only the PATH environment variable is backed up
- **Shell Only**: Only shell configuration files are backed up
- **Switch**: Toggle between Path Only and Shell Only modes

You can set the backup mode with the `--backup-mode` flag:

```bash
pathmaster --backup-mode path add ~/bin
```

## Listing Backups

You can view all available backups with:

```bash
pathmaster history
```

This command displays a chronological list of backups with their timestamps, allowing you to choose which backup to restore.

## Restoring from Backups

To restore from a backup, use:

```bash
# Restore from the most recent backup
pathmaster restore

# Restore from a specific backup by timestamp
pathmaster restore --timestamp 20250402150432
```
