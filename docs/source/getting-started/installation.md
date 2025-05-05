# Installation Guide

## System Requirements

- Rust toolchain (for installation via cargo)
- Linux operating system
- Supported shell (bash, zsh, fish, tcsh, or ksh)

## Installation Methods

### Using Cargo

The recommended way to install pathmaster is through cargo:

```bash
cargo install pathmaster
```

This will:

1. Download the latest version
2. Compile the source code
3. Install the binary in your cargo bin directory

### Building from Source

1. Clone the repository:

```bash
git clone https://github.com/jwliles/pathmaster.git
cd pathmaster
```

2. Build the release version:

```bash
cargo build --release
```

3. The binary will be available at:

```bash
./target/release/pathmaster
```

4. Optionally, install system-wide:

```bash
cargo install --path .
```

## Verifying Installation

After installation, verify pathmaster is working:

```bash
pathmaster --version
pathmaster --help
```

## Shell Integration

Pathmaster automatically detects and configures your shell:

### Supported Shell Configurations

- bash: ~/.bashrc
- zsh: ~/.zshrc
- fish: ~/.config/fish/config.fish
- tcsh: ~/.tcshrc
- ksh: ~/.kshrc

### Initial Setup

No manual configuration is needed. Pathmaster will:

1. Detect your current shell
2. Locate the appropriate configuration file
3. Make necessary updates automatically
4. Create backups before any modifications

## Troubleshooting Installation

### Common Issues

1. Cargo not found

   - Solution: Install Rust and Cargo from https://rustup.rs/

2. Permission errors

   - Solution: Check directory permissions or use sudo for system directories

3. Shell not detected
   - Solution: Ensure SHELL environment variable is set correctly

### Getting Help

- [Check the documentation](https://jwliles.github.io/pathmaster)
- [Report issues](https://github.com/jwliles/pathmaster/issues)
- Man page: `man pathmaster`
