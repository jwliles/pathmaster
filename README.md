# pathmaster

A powerful command-line tool for managing your system's PATH environment variable, providing safe and efficient PATH manipulation with automatic backups and validation.

Version: 0.2.9

[![Crates.io](https://img.shields.io/crates/v/pathmaster.svg)](https://crates.io/crates/pathmaster)
[![Documentation](https://img.shields.io/badge/docs-ReadTheDocs-blue.svg)](https://pathmaster.readthedocs.io/)
[![API Docs](https://docs.rs/pathmaster/badge.svg)](https://docs.rs/pathmaster)

## Features

- Safe PATH manipulation
- Smart shell configuration management
- Comprehensive validation and error checking
- Basic error prevention

## Upcoming Features

- Enhanced backup system with multiple formats (JSON/TOML/plain text)
- User-defined backup locations
- Format conversion utilities
- Flexible backup modes for different needs
- Complete shell configuration management

See our [Roadmap](https://pathmaster.readthedocs.io/en/latest/ROADMAP.html) for more details.

## Quick Start

### Installation

```bash
cargo install pathmaster
```

### Man Page Installation

After installing with Cargo, you can install the man page with:

```bash
pathmaster-install-man $HOME/.local
```

Or build from source:

```bash
git clone https://github.com/jwliles/pathmaster.git
cd pathmaster
cargo build --release
```

### Basic Usage

```bash
# Add a directory to PATH
pathmaster add ~/bin

# List current PATH entries
pathmaster list

# Remove invalid entries
pathmaster flush

# Show backup history
pathmaster history
```

## Documentation

- **User Documentation**: [https://pathmaster.readthedocs.io/](https://pathmaster.readthedocs.io/)
- **API Documentation**: [https://docs.rs/pathmaster](https://docs.rs/pathmaster)
- **Man Page**: `man pathmaster`
- **Command Help**: `pathmaster --help`

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on:

- Code of Conduct
- Development process
- Bug reports
- Feature requests
- Pull requests

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
