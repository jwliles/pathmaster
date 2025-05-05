Go Usage Differences
===================

This document outlines the differences in usage between the Rust and Go implementations of Pathmaster.

Command Line Interface
---------------------

The Go implementation aims to maintain the same CLI interface as the Rust version to ensure a smooth transition for users. However, there may be some subtle differences:

Installation
~~~~~~~~~~~

**Rust Version:**

.. code-block:: bash

   cargo install pathmaster

**Go Version (future):**

.. code-block:: bash

   # Via Go install
   go install github.com/jwliles/pathmaster/cmd/pathmaster@latest

   # Via binary releases (after v0.3.0)
   # Download appropriate binary from GitHub releases

Man Page
~~~~~~~~

**Rust Version:**

.. code-block:: bash

   pathmaster-install-man $HOME/.local

**Go Version:**
Man pages will be included in binary releases or can be generated from source.

New Features in Go Version
-------------------------

The Go implementation will include several new features that are not available in the Rust version:

Enhanced Backup System
~~~~~~~~~~~~~~~~~~~~~

- Multiple backup formats (JSON, TOML, text)
- User-defined backup locations
- Backup validation
- Format conversion utilities

Interactive TUI Mode
~~~~~~~~~~~~~~~~~~~

.. code-block:: bash

   # Enter interactive TUI mode
   pathmaster tui

The TUI mode will provide:

- Visual path management
- Interactive editing
- Validation visualization
- Configuration management

Configuration Profiles
~~~~~~~~~~~~~~~~~~~~~

.. code-block:: bash

   # Create a new profile
   pathmaster profile create development

   # Switch to a profile
   pathmaster profile use development

   # List available profiles
   pathmaster profile list

Behavior Differences
------------------

While we aim for feature parity, there might be some behavioral differences between the implementations:

1. **Error Messages**: Error messages may be more detailed in the Go version
2. **Shell Detection**: The algorithm for detecting the current shell might be slightly different
3. **Performance**: The Go version may have different performance characteristics

Compatibility
------------

The Go version will be compatible with backup files created by the Rust version. If there are format differences, conversion utilities will be provided.

Feedback on Differences
---------------------

As the Go implementation progresses, please report any unexpected differences in behavior or functionality through:

- GitHub Issues: `https://github.com/jwliles/pathmaster/issues <https://github.com/jwliles/pathmaster/issues>`_
- Documentation Suggestions: Create a pull request for these documentation files