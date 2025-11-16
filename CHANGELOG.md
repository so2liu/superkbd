# Changelog

All notable changes to SuperKBD will be documented in this file.

## [0.1.3] - 2025-11-16

### Changed
- Migrated releases to public repository (superkbd-releases)
- Added build caching for faster CI/CD (Rust cargo + Bun dependencies)
- Updated updater endpoint to point to public releases repo

### Technical
- Source code remains private, releases are now public
- Build time reduced by ~50% with dependency caching
- Following kb-example pattern for private code + public artifacts

## [0.1.2] - 2025-11-16

### Fixed
- Auto-updater configuration: Added `createUpdaterArtifacts` to properly generate update manifests

## [0.1.1] - 2025-11-16

### Added
- Auto-update functionality with signed releases
- Comprehensive CI/CD pipeline with GitHub Actions

### Changed
- Optimized build process (removed x86 macOS support)
- Improved branching strategy (develop/main)

### Fixed
- Minor bug fixes and improvements

## [0.1.0] - 2025-11-16

### Added
- Initial release
- Raycast-style clipboard manager
- Fast auto-paste functionality (~60ms)
- macOS global shortcut (Alt+I)
- SQLite-based clipboard history
- Search and favorite功能
