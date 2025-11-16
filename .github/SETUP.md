# GitHub CI/CD Setup Guide

This guide explains how to set up the GitHub repository for automated builds and releases with auto-update support.

## Prerequisites

- GitHub account with access to the repository
- Private signing key generated for Tauri updater

## GitHub Secrets Configuration

The CI/CD workflow requires two GitHub secrets to sign the update packages. Navigate to your repository settings: **Settings > Secrets and variables > Actions** and add the following secrets:

### 1. TAURI_SIGNING_PRIVATE_KEY

This is the content of your private key file.

**How to get the value:**
```bash
cat ~/.tauri/superkbd.key
```

Copy the entire output and paste it as the secret value.

**Important:** Keep this key secure! If you lose it, you won't be able to sign updates and users won't be able to update their applications.

### 2. TAURI_SIGNING_PRIVATE_KEY_PASSWORD

This is the password used when generating the signing key.

**For this project:** `superkbd-signing-key-2024`

**Important:** Store this password securely. Without it, the CI/CD pipeline cannot sign the update packages.

## Release Process

The GitHub Actions workflow is triggered in two ways:

### 1. Tag-based Release (Recommended)

Create and push a version tag:

```bash
# Update version in package.json and tauri.conf.json
# Then create a tag
git tag v0.1.0
git push origin v0.1.0
```

This will automatically:
1. Build the app for macOS (Intel + Apple Silicon) and Windows
2. Create a GitHub release
3. Upload signed installers
4. Generate update manifests for auto-update

### 2. Manual Workflow Dispatch

Go to **Actions > Release > Run workflow** to manually trigger a build.

## How Auto-Update Works

1. **User opens the app**: The app checks the configured endpoint for updates
2. **Update available**: If a newer version is found, a dialog prompts the user
3. **User accepts**: The update is downloaded, verified with the public key, and installed
4. **App restarts**: The new version launches automatically

**Endpoint:** `https://github.com/so2liu/superkbd/releases/latest/download/latest.json`

**Public Key:** Embedded in `tauri.conf.json`

## Build Artifacts

Each release creates the following artifacts:

### macOS
- `SuperKBD_x.y.z_aarch64.dmg` - Apple Silicon installer
- `SuperKBD_x.y.z_x64.dmg` - Intel installer
- `.tar.gz` files - For auto-update

### Windows
- `SuperKBD_x.y.z_x64-setup.exe` - Windows installer
- `.nsis.zip` files - For auto-update

## Testing the Workflow

1. Make a small change (e.g., bump version to 0.1.1)
2. Create a tag: `git tag v0.1.1 && git push origin v0.1.1`
3. Watch the Actions tab for build progress
4. Check the Releases page for the published release

## Troubleshooting

### Build fails with signing error
- Verify both secrets are correctly set in repository settings
- Ensure there are no extra spaces or newlines in the secret values

### Update not working for users
- Verify the `endpoints` URL in `tauri.conf.json` is correct
- Check that the release is published (not draft)
- Ensure the `latest.json` file exists in the release assets

### Platform-specific build fails
- Check the workflow logs for specific error messages
- macOS builds require Xcode and developer certificates
- Windows builds require NSIS installer

## Security Notes

1. **Never commit the private key** to the repository
2. **Keep the password secure** - store it in a password manager
3. **Private repository** ensures code remains confidential while releases are public
4. **Signed updates** prevent malicious updates from being installed

## Repository Structure

```
.github/
  workflows/
    release.yml       # Main CI/CD workflow
  SETUP.md           # This file

src-tauri/
  tauri.conf.json    # Contains public key and update endpoint
  Cargo.toml         # Includes tauri-plugin-updater

~/.tauri/
  superkbd.key       # Private key (local only, never commit!)
  superkbd.key.pub   # Public key (embedded in config)
```

## Next Steps

1. Set up the GitHub secrets as described above
2. Test the workflow with a v0.1.0 tag
3. Verify the release appears in the Releases page
4. Install the app and test auto-update with a v0.1.1 release
