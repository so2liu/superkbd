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
