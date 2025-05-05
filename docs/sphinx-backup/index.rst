Welcome to Pathmaster Documentation
===============================

.. image:: https://img.shields.io/crates/v/pathmaster.svg
   :alt: Crates.io
   :target: https://crates.io/crates/pathmaster

.. image:: https://docs.rs/pathmaster/badge.svg
   :alt: Documentation
   :target: https://docs.rs/pathmaster

A powerful command-line tool for managing your system's PATH environment variable, providing safe and efficient PATH manipulation with automatic backups and validation.

Features
--------

- Safe PATH manipulation
- Smart shell configuration management
- Comprehensive validation and error checking
- Basic error prevention

Upcoming Features
----------------

- Enhanced backup system with multiple formats (JSON/TOML/plain text)
- User-defined backup locations
- Format conversion utilities
- Flexible backup modes for different needs
- Complete shell configuration management

Contents
--------

.. toctree::
   :maxdepth: 2
   :caption: Getting Started
   
   src/introduction
   src/getting-started/installation
   src/getting-started/quick-start
   src/getting-started/basic-usage

.. toctree::
   :maxdepth: 2
   :caption: Core Functionality
   
   src/commands/overview
   src/commands/path-management
   src/commands/validation
   src/commands/backup

.. toctree::
   :maxdepth: 2
   :caption: Features
   
   src/features/overview
   src/features/backup-system
   src/features/shell-support
   src/features/validation

.. toctree::
   :maxdepth: 2
   :caption: User Guides
   
   src/guides/error-handling
   src/guides/troubleshooting
   src/guides/system-integration
   src/guides/migration

.. toctree::
   :maxdepth: 2
   :caption: Project
   
   src/ROADMAP

Indices and tables
==================

* :ref:`genindex`
* :ref:`search`