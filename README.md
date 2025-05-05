# pathmaster

A powerful command-line tool for managing your system's PATH environment variable, providing safe and efficient PATH manipulation with automatic backups and validation.

> **Note:** This project is being migrated from Rust to Go. See [GO_MIGRATION_PLAN.md](GO_MIGRATION_PLAN.md) for details. The current Rust implementation remains fully functional.

Version: 0.2.9 (Rust) | Dev (Go)

[![Crates.io](https://img.shields.io/crates/v/pathmaster.svg)](https://crates.io/crates/pathmaster)
![Read the Docs](https://img.shields.io/readthedocs/pathmaster)

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

### Development Branching Strategy

> **Important for contributors:** During the Go migration, we're using a special branching strategy:
> 
> - `main` branch contains the stable Rust implementation
> - `feature/go-migration` branch is the primary development branch for Go implementation
> - New features and fixes for the Go version should branch from `feature/go-migration`
> - Rust version fixes should still branch from `main`
> 
> This approach allows us to maintain the Rust version while developing the Go implementation in parallel.

## Requirements

### Rust Version
- Rust (Minimum supported version: 1.68.0)
- GNU/Linux or other free operating system
- Standard system libraries

### Go Version (Under Development)
- Go 1.20+
- GNU/Linux or other free operating system
- Standard system libraries

**Note**: Pathmaster is developed exclusively for free operating systems. It is not officially tested or supported on proprietary platforms.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
