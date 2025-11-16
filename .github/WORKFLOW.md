# CI/CD Workflow Documentation

## Overview

SuperKBD uses a streamlined CI/CD pipeline optimized for fast feedback and efficient resource usage.

## Workflow Files

### 1. test.yml - Fast Testing on develop
**Trigger**: Push to `develop` branch or PR to `develop`/`main`
**Duration**: ~2-3 minutes
**Purpose**: Fast feedback loop for daily development

**Jobs**:
- `test-rust`: Run Rust unit tests on multiple platforms
- `test-frontend`: Run frontend tests with Bun
- `lint`: Run svelte-check and linting

**Why it's fast**:
- No compilation needed (tests only)
- Runs in parallel across platforms
- Caches Cargo dependencies

### 2. build.yml - Verification Builds on main
**Trigger**: Push to `main` branch
**Duration**: ~10-15 minutes
**Purpose**: Verify that code can be built successfully

**Jobs**:
- `build-tauri`: Build for macOS ARM and Windows x64
  - Compiles application
  - Creates unsigned bundles
  - Uploads artifacts (7-day retention)
  - **Does NOT create release**

**Use case**: Verify PR merges compile correctly before tagging a release

### 3. release.yml - Release Creation on Tags
**Trigger**: Push tag matching `v*` pattern
**Duration**: ~10-15 minutes
**Purpose**: Create signed, published releases

**Jobs**:
- `create-release`: Create GitHub release (draft)
- `build-tauri`: Build signed binaries for:
  - macOS ARM (Apple Silicon)
  - Windows x64
- `publish-release`: Publish the draft release

**Artifacts produced**:
- macOS: `.dmg` installer + `.tar.gz` for auto-update
- Windows: `.exe` installer + `.nsis.zip` for auto-update
- Update manifest: `latest.json`

## Platform Support

| Platform | Architecture | Support Level | Notes |
|----------|-------------|---------------|-------|
| macOS | ARM (Apple Silicon) | Primary | Main development platform |
| macOS | x86_64 (Intel) | **Removed** | No longer built |
| Windows | x64 | Secondary | Full support |

**Rationale for removing x86 macOS**:
- Apple Silicon has been the standard since 2020
- Reduces build time by ~30%
- Most users have transitioned to ARM
- Can be re-enabled if needed by uncommenting in workflow

## Branching Strategy

```
develop (default)          main (protected)         tags
    │                           │                     │
    ├─ feature work            │                     │
    ├─ tests run (2-3min) ─────┤                     │
    │                           ├─ merge              │
    │                           ├─ build (10-15min)  │
    │                           ├─ tag created ──────>│
    │                           │                     ├─ release build
    │                           │                     └─ publish
```

### Branch Protection (Recommended)

**For `main` branch**:
- Require PR reviews
- Require status checks to pass
- Require branches to be up to date
- No direct pushes (except for version bumps before tagging)

**For `develop` branch**:
- Allow direct pushes (fast iteration)
- Run tests on every push
- Require tests to pass before merging to main

## Workflow Comparison

| Aspect | develop (Tests) | main (Build) | Tag (Release) |
|--------|----------------|--------------|---------------|
| **Trigger** | Push to develop | Push to main | Tag v* |
| **Duration** | 2-3 min | 10-15 min | 10-15 min |
| **Tests** | Yes | Yes (implicit) | Yes (implicit) |
| **Build** | No | Yes | Yes |
| **Signing** | No | No | Yes |
| **Release** | No | No | Yes |
| **Cost** | Low | Medium | Medium |

## Cost Optimization

**GitHub Actions minutes saved per day** (assuming 10 commits/day):

- **Before** (all builds on every push): 10 × 15min × 2 platforms = 300 min/day
- **After** (tests only on develop): 10 × 3min = 30 min/day
- **Savings**: 270 minutes/day (90% reduction!)

Only building on main merges (1-2/day) and releases (0-1/day) dramatically reduces CI costs.

## Example Workflows

### Daily Development
```bash
# On develop branch
git pull origin develop
# Make changes
git add .
git commit -m "feat: add new feature"
git push origin develop

# Tests run automatically (~3 min)
# No build, fast feedback
```

### Preparing a Release
```bash
# Merge develop to main
git checkout main
git merge develop
git push origin main

# Build runs automatically (~15 min)
# Verify it succeeded

# If build passes, create release
git tag v0.2.0
git push origin v0.2.0

# Release workflow runs
# Signed installers published
```

### Hotfix on main
```bash
# For urgent fixes
git checkout main
# Make fix
git commit -m "fix: critical bug"
git push origin main

# Build runs to verify
# Then tag for release
git tag v0.1.1
git push origin v0.1.1
```

## Monitoring

**View workflow runs**:
```bash
gh run list
gh run view <run-id>
gh run watch <run-id>
```

**Check workflow status**:
- develop: https://github.com/so2liu/superkbd/actions/workflows/test.yml
- main: https://github.com/so2liu/superkbd/actions/workflows/build.yml
- releases: https://github.com/so2liu/superkbd/actions/workflows/release.yml

## Troubleshooting

**Tests failing on develop**:
- Run `bun test` and `cargo test` locally
- Fix issues before pushing

**Build failing on main**:
- Check dependency versions
- Ensure Rust/Bun versions match CI
- Review build logs in Actions tab

**Release failing**:
- Verify GitHub secrets are set correctly
- Check that version numbers are updated
- Ensure tag matches version in package.json

## Future Enhancements

Possible improvements to consider:

1. **Add Linux build** (if user base grows)
2. **Nightly builds** from develop for testing
3. **Performance benchmarks** in CI
4. **Automated changelog** generation from commits
5. **Canary releases** for beta testers

## References

- GitHub Actions: https://docs.github.com/en/actions
- Tauri CI: https://tauri.app/v1/guides/building/cross-platform
- Bun in CI: https://bun.sh/docs/cli/test
