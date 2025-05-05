# Pathmaster Roadmap

This document outlines the current and planned features for Pathmaster, providing visibility into the project's direction and future development.

## Current Features (v0.2.7)

### Path Management
- ✅ Add directories to PATH
- ✅ Remove directories from PATH
- ✅ List current PATH entries
- ✅ Check paths for validity

### Validation
- ✅ Identify invalid path entries
- ✅ Flush invalid entries with confirmation
- ✅ Prevent duplicates and circular references

### Shell Support
- ✅ Basic shell configuration detection
- ✅ Support for major shells (Bash, Zsh, Fish)
- ✅ Configuration file path detection

### Basic Backup System (Current Implementation)
- ✅ Create basic JSON backups of PATH
- ✅ Restore from backups
- ✅ View backup history
- ⚠️ Limited format options
- ⚠️ Fixed backup location
- ⚠️ Basic shell configuration handling

## Backup System Improvements (v0.3.0)

The existing backup system will be enhanced to provide more robust and flexible functionality:

### Core Improvements
- Multiple backup format support (JSON/TOML/plain text)
- User-defined backup locations
- Format conversion utilities
- Shell configuration preservation
- Backup validation and verification

### Implementation Plan

#### Backup Creation
- Configurable format handlers
- Custom location management
- Improved error handling
- Backup compression options

#### Storage Management
- Format-specific serialization
- Location validation
- Space management
- Cleanup utilities

#### Restoration
- Point-in-time restoration
- Partial restoration (PATH-only, shell-only)
- Backup verification
- Conflict resolution

## Future Versions

### Version 0.4.0
- Interactive terminal UI
- Enhanced error handling and reporting
- Performance optimizations
- Command output formatting options

### Version 0.5.0
- Advanced shell framework integrations
- Profile management for different environments
- System-wide vs. user-specific management

### Version 1.0.0
- Complete core functionality stabilization
- Full test coverage
- Comprehensive documentation
- Performance benchmarks

## Future Considerations

- Cross-platform synchronization
- Plugin system
- GUI application
- Cloud backup integration

---

Legend:
- ✅ Implemented
- ⚠️ Implemented but needs improvement
- 🔄 In progress
- 📅 Planned