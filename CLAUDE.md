# SuperKBD Development Notes

## Project Overview
Tauri-based clipboard manager with Raycast-style auto-paste functionality.

## Key Technical Decisions

### macOS App Activation Performance

When switching focus between apps on macOS, performance matters significantly:

**Avoid: AppleScript for app activation**
```rust
// Slow: ~112ms
let script = format!(r#"tell application "{}" to activate"#, app_name);
Command::new("osascript").arg("-e").arg(&script).output();
```

**Use: `open -a` command with spawn**
```rust
// Fast: ~6ms
Command::new("open")
    .arg("-a")
    .arg(&app_name)
    .spawn(); // Use spawn(), not output() - don't wait for completion
```

**Performance comparison:**
- AppleScript (`osascript`): 112ms
- `open -a` with `spawn()`: 6ms
- **~95% faster!**

**Why `open -a` is better:**
1. Direct system API call, no script parsing overhead
2. Using `.spawn()` makes it asynchronous - we don't wait for completion
3. Works correctly with multiple windows of the same app (returns to last active window)

### Keyboard Simulation on macOS

For fast keyboard event simulation, use CGEvent API:

```rust
use core_graphics::event::{CGEvent, CGEventFlags, CGEventTapLocation, CGKeyCode};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)?;
const V_KEYCODE: CGKeyCode = 9;

let key_down = CGEvent::new_keyboard_event(source.clone(), V_KEYCODE, true)?;
key_down.set_flags(CGEventFlags::CGEventFlagCommand);
key_down.post(CGEventTapLocation::HID);
```

Much faster than enigo or other cross-platform solutions (~20ms total for paste).

### Tauri Development Workflow

**Common pitfalls:**

1. **Multiple dev servers**: Easy to accidentally run multiple `tauri dev` instances
   - Kill all before starting: `pkill -9 -f "tauri dev"`
   - Check running servers: `ps aux | grep "tauri dev"`

2. **Code reload**: Tauri apps don't refresh like web apps
   - Need to **Cmd+Q** (quit completely) and let dev server restart
   - **Cmd+R** doesn't work in Tauri apps
   - Auto-reload works for Rust changes, but need clean state

3. **Viewing logs**: Use `tee` to capture output
   ```bash
   bun run tauri dev 2>&1 | tee /tmp/tauri-debug.log
   ```

### Focus Management

Store the previous app **before** showing the SuperKBD window:

```rust
// Get frontmost app BEFORE showing our window
let target_app = window::get_previous_app();

// Hide our window
window.hide()?;

// Immediately activate previous app (don't wait for natural focus return)
Command::new("open").arg("-a").arg(&app_name).spawn()?;

// Small delay for app to receive focus
tokio::time::sleep(Duration::from_millis(30)).await;

// Then paste
```

**Timing optimization:**
- Total paste time: ~60ms (from Enter to paste complete)
- Clipboard copy: ~1ms
- App activation: ~6ms (spawn)
- Focus delay: 30ms
- Paste execution: ~20ms

### Performance Profiling

Always add detailed timing logs for performance-critical paths:

```rust
use std::time::Instant;

let start = Instant::now();
println!("T+{}ms: Step description", start.elapsed().as_millis());
```

This helped identify that AppleScript was the bottleneck (112ms out of 149ms total).

## Common Issues

### Issue: Paste feels slow compared to Raycast
**Solution**: Replace AppleScript app activation with `open -a` + spawn

### Issue: Focus doesn't return to correct window
**Solution**: macOS handles this automatically with `open -a` - it returns to the last active window of that app

### Issue: Performance logs not showing up
**Solution**: Multiple dev servers running simultaneously. Kill all and restart cleanly.

## Dependencies

- `core-graphics = "0.23"` - For fast keyboard simulation on macOS
- `arboard` - Cross-platform clipboard access
- `tauri` - Desktop app framework

## Best Practices

1. **Always profile before optimizing**: Add timing logs to identify actual bottlenecks
2. **Use platform-specific APIs for performance**: CGEvent on macOS is much faster than cross-platform solutions
3. **Prefer `.spawn()` over `.output()`**: When you don't need to wait for command completion
4. **Test edge cases**: Multiple windows of same app, different target apps, etc.

## CI/CD and Deployment

### GitHub Actions Setup

The project uses GitHub Actions for automated builds and releases:

**Workflow file**: `.github/workflows/release.yml`

**Trigger**: Push tags matching `v*` pattern (e.g., `v0.1.0`)

**Platforms**:
- macOS (Apple Silicon and Intel)
- Windows (x64)

### Release Process

1. Update version in `package.json` and `tauri.conf.json`
2. Commit changes
3. Create and push a tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
4. GitHub Actions automatically:
   - Builds for all platforms
   - Signs the releases
   - Creates GitHub release
   - Uploads installers and update manifests

### Auto-Update System

**How it works**:
1. App checks endpoint on startup: `https://github.com/so2liu/superkbd/releases/latest/download/latest.json`
2. Compares current version with latest available
3. Shows update dialog if newer version exists
4. Downloads and verifies update signature
5. Installs and restarts automatically

**Security**:
- Updates are signed with private key (stored in GitHub secrets)
- App verifies signature with embedded public key
- Only validated updates can be installed

**Configuration**:
```json
{
  "plugins": {
    "updater": {
      "active": true,
      "endpoints": ["https://github.com/so2liu/superkbd/releases/latest/download/latest.json"],
      "dialog": true,
      "pubkey": "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEJCQkEwNDhCNENEN0M2ODYKUldTR3h0ZE1pd1M2dXo2NzJSNSswbU8yOXpqMVZoZ3BiYStSbSt1Wk9CTnVzcG50eVJjcDhHYWwK"
    }
  }
}
```

### GitHub Secrets Required

Set these in repository settings (Settings > Secrets and variables > Actions):

1. **TAURI_SIGNING_PRIVATE_KEY**: Content of `~/.tauri/superkbd.key`
2. **TAURI_SIGNING_PRIVATE_KEY_PASSWORD**: Password used when generating key

**Generating new keys** (if needed):
```bash
bunx tauri signer generate --password "your-password" -w ~/.tauri/superkbd.key
```

### Build Artifacts

Each release produces:

**macOS**:
- `SuperKBD_x.y.z_aarch64.dmg` - Apple Silicon installer
- `SuperKBD_x.y.z_x64.dmg` - Intel installer
- `.tar.gz` files - For auto-update

**Windows**:
- `SuperKBD_x.y.z_x64-setup.exe` - Installer
- `.nsis.zip` files - For auto-update

### Repository Structure

- **Private repository**: Code remains confidential
- **Public releases**: Users can download installers without repo access
- **Auto-update**: Works from public GitHub releases

### Build Performance Optimizations

The project includes several optimizations to speed up CI/CD builds, especially for Windows:

**1. Rust Compilation Cache**
- Uses `Swatinem/rust-cache@v2` to cache Rust dependencies and build artifacts
- Significantly reduces rebuild times (first build ~10-15 min, cached builds ~3-5 min)
- Cache is shared across workflow runs for the same branch

**2. Cargo Build Configuration** (`.cargo/config.toml`)
- `jobs = 0`: Uses all available CPU cores for parallel compilation
- Windows-specific target configuration for optimal compilation

**3. Release Profile Optimization** (`src-tauri/Cargo.toml`)
- `codegen-units = 16`: More units = faster parallel compilation
- `lto = "thin"`: Thin LTO provides good optimization with faster compile times than full LTO
- `opt-level = "z"`: Size optimization is faster to compile than `-O3` while producing smaller binaries
- `strip = true`: Removes debug symbols to reduce binary size

**Expected build times**:
- First build (no cache): 10-15 minutes per platform
- Subsequent builds (cached): 3-5 minutes per platform
- The Rust cache persists for 7 days of inactivity

**Cache invalidation**:
- Cache automatically refreshes when `Cargo.lock` changes
- Manual cache clearing: Delete caches in Actions > Caches

### Troubleshooting Releases

**Build fails with signing error**:
- Verify GitHub secrets are set correctly
- Check private key format (should include header/footer)
- Ensure password matches the one used during key generation

**Auto-update not working**:
- Verify `endpoints` URL in `tauri.conf.json` is correct
- Check release is published (not draft)
- Ensure `latest.json` exists in release assets

**Platform-specific build fails**:
- Check workflow logs for detailed error messages
- macOS builds require valid app identifiers
- Windows builds need NSIS installer properly configured

For detailed setup instructions, see `.github/SETUP.md`
