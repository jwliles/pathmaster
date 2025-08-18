# Changelog

## [0.3.0] - 2025-08-17

### Added
- Interactive multi-select removal of PATH entries using a CLI prompt (via `dialoguer`). If no arguments are provided to the remove command, you can now select one or more entries to remove from a list.
- The old argument-based removal remains as a fallback.

### Fixed
- Robust handling and formatting of multi-line and append-form (`path+=(`) path arrays in `.zshrc`.
- No more duplicated last line when updating `.zshrc`.
- Preserves assignment operator (`=` or `+=`) when updating path arrays.

### Improved
- Multi-line formatting for path arrays in `.zshrc` for better readability.
- Comprehensive test coverage for edge cases in path array handling.
