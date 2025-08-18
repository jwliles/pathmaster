# Pathmaster Roadmap

This document outlines the current and planned features for Pathmaster, providing visibility into the project's direction and future development.

## Current Features (v0.3.0)

### Path Management
- ✅ Add directories to PATH
- ✅ Remove directories from PATH (with interactive multi-select)
- ✅ List current PATH entries
- ✅ Check paths for validity

### Interactive Features
- ✅ Interactive multi-select removal functionality
- ✅ User-friendly prompts and confirmations
- ✅ Enhanced user experience for bulk operations

### Validation
- ✅ Identify invalid path entries
- ✅ Flush invalid entries with confirmation
- ✅ Prevent duplicates and circular references

### Shell Support
- ✅ Enhanced shell configuration detection
- ✅ Support for major shells (Bash, Zsh, Fish, Tcsh, Ksh)
- ✅ Improved zshrc handling with multi-line support
- ✅ Better configuration file management

### Backup System
- ✅ Create JSON backups of PATH and shell configuration
- ✅ Restore from backups
- ✅ View backup history
- ✅ Multiple backup modes (default, path, shell, switch)

## Planned Improvements (v0.4.0)

### Enhanced Backup System
- Multiple backup format support (JSON/TOML/plain text)
- User-defined backup locations
- Format conversion utilities
- Backup validation and verification
- Backup compression options

### Terminal UI Enhancements
- Interactive terminal UI improvements
- Better command output formatting
- Enhanced error handling and reporting
- Performance optimizations

## Future Versions

### Version 0.5.0
- Advanced shell framework integrations
- Profile management for different environments
- System-wide vs. user-specific management
- Configuration management improvements

### Version 1.0.0
- Complete core functionality stabilization
- Full test coverage
- Comprehensive documentation
- Performance benchmarks
- API stability guarantees

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