---
date_created: 2025-10-02T11-25-26
date_updated: 2025-08-30T02-49-20
timestamp: 1759404326128
title: CHANGELOG
id: f65e7ba1-7e1b-4779-9144-96cea12e1809
hash: 9a8697d2d884bb590589635cda3af08c9b322e2d389eb31bd02edd347e1d1e11
---
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
