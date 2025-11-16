# Changelog

All notable changes to SuperKBD will be documented in this file.

## [0.1.6] - 2025-11-16

### Changed
- **Open source**: Repository is now public
- **Simplified CI/CD**: Single-repository architecture (no separate releases repo)
- **Windows build optimization**: Added sccache for ~50% faster Rust compilation
- **Updater endpoint**: Now points to main repository releases

### Technical
- Added sccache caching for Windows builds
- Set CARGO_BUILD_JOBS=4 for parallel compilation
- Removed cross-repository publishing logic
- Simplified release workflow to use tauri-action directly

## [0.1.5] - 2025-11-16

### Fixed
- Fixed shell syntax error in release workflow

## [0.1.4] - 2025-11-16

### Fixed
- Eliminated GitHub Actions artifact storage dependency to avoid quota issues
- Build jobs now upload directly to public releases repository

### Technical
- Workflow rewritten: create-release job runs first, build jobs upload to existing release
- No intermediate artifact storage required

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
