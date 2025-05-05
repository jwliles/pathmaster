Go Migration Overview
===================

Pathmaster is being migrated from Rust to Go to provide enhanced features and a more interactive experience. This document provides an overview of the migration process, goals, and timeline.

Migration Goals
--------------

1. Complete feature parity with Rust v0.2.9
2. Implement backup system improvements (v0.3.0 features)
3. Add interactive TUI (v0.4.0 feature)
4. Maintain the same CLI interface for user continuity
5. Ensure all tests pass with equivalent or better coverage
6. Provide clear documentation for both versions

Timeline
--------

The migration is planned in the following phases:

Phase 1: Core Functionality (v0.3.0 equivalent)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

- Path management functions
- Shell detection and integration
- CLI command structure using Cobra or similar
- Validation logic
- Backup system improvements (multiple formats, user-defined locations, etc.)

Phase 2: Interactive TUI (v0.4.0)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

- Interactive shell for managing PATH entries
- Visualization of path entries and their validation status
- Command panel with keyboard shortcuts
- Configuration editor

Phase 3: Advanced Features (v0.5.0)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

- Profile management
- Advanced shell integration
- System-wide vs. user-specific management

Current Status
-------------

The Go implementation is currently in early development. You can track the status of the migration in the :doc:`Feature Parity Status <feature-parity>` document.

Development Repositories
-----------------------

During the migration, both implementations will co-exist:

- The Rust implementation is maintained in the ``rust/`` directory of the main repository
- The Go implementation is being developed in the root directory
- Eventually, the Rust version will be moved to its own repository and maintained separately

Contributing to the Go Version
-----------------------------

If you'd like to contribute to the Go implementation, please note the special branching strategy:

- ``main`` branch contains the stable Rust implementation
- ``feature/go-migration`` branch is the primary development branch for Go implementation
- New features and fixes for the Go version should branch from ``feature/go-migration``

For more details about the migration plan, see the `GO_MIGRATION_PLAN.md <https://github.com/jwliles/pathmaster/blob/feature/go-migration/GO_MIGRATION_PLAN.md>`_ file in the repository.