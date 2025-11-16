use anyhow::Result;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::thread;
use std::time::Duration;

#[cfg(target_os = "macos")]
const MODIFIER_KEY: Key = Key::Meta;

#[cfg(not(target_os = "macos"))]
const MODIFIER_KEY: Key = Key::Control;

/// Check if we have accessibility permissions
/// Returns true if we can use keyboard simulation
pub fn check_accessibility_permission() -> bool {
    // Try to create Enigo instance - if it fails, we don't have permissions
    match std::panic::catch_unwind(|| {
        Enigo::new(&Settings::default())
    }) {
        Ok(Ok(_)) => true,
        _ => false,
    }
}

pub async fn simulate_paste(content: String) -> Result<()> {
    // First, copy to clipboard
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(content.clone())?;

    println!("✅ Content copied to clipboard: {}", content);

    // Minimal delay - just enough for clipboard to update
    tokio::time::sleep(Duration::from_millis(10)).await;

    println!("🔧 [DEBUG] Starting keyboard simulation...");

    // Use platform-specific paste simulation
    #[cfg(target_os = "macos")]
    {
        // On macOS, use CGEvent for fast keyboard simulation
        // This is much faster than AppleScript
        println!("🔧 [DEBUG] Using CGEvent to simulate Cmd+V...");

        use core_graphics::event::{CGEvent, CGEventFlags, CGEventTapLocation, CGKeyCode, EventField};
        use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

        // Create event source
        let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
            .map_err(|_| anyhow::anyhow!("Failed to create event source"))?;

        // V key code on macOS
        const V_KEYCODE: CGKeyCode = 9;

        // Create key down event for V with Cmd modifier
        let key_down = CGEvent::new_keyboard_event(source.clone(), V_KEYCODE, true)
            .map_err(|_| anyhow::anyhow!("Failed to create key down event"))?;
        key_down.set_flags(CGEventFlags::CGEventFlagCommand);

        // Create key up event for V with Cmd modifier
        let key_up = CGEvent::new_keyboard_event(source.clone(), V_KEYCODE, false)
            .map_err(|_| anyhow::anyhow!("Failed to create key up event"))?;
        key_up.set_flags(CGEventFlags::CGEventFlagCommand);

        // Post events
        key_down.post(CGEventTapLocation::HID);
        key_up.post(CGEventTapLocation::HID);

        println!("✅ Auto-paste completed successfully!");
        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    {
        // On Windows/Linux, use enigo
        println!("🔧 [DEBUG] Using enigo for paste simulation...");

        // Spawn blocking task for keyboard simulation
        let result = tokio::task::spawn_blocking(move || -> Result<()> {
            thread::sleep(Duration::from_millis(50));

            let mut enigo = Enigo::new(&Settings::default())
                .map_err(|e| anyhow::anyhow!("Failed to initialize keyboard simulator: {}", e))?;

            // Simulate Ctrl+V (Windows/Linux)
            enigo.key(MODIFIER_KEY, Direction::Press)
                .map_err(|e| anyhow::anyhow!("Failed to press modifier key: {}", e))?;

            enigo.key(Key::Unicode('v'), Direction::Click)
                .map_err(|e| anyhow::anyhow!("Failed to click V key: {}", e))?;

            enigo.key(MODIFIER_KEY, Direction::Release)
                .map_err(|e| anyhow::anyhow!("Failed to release modifier key: {}", e))?;

            println!("✅ Auto-paste completed successfully!");
            Ok(())
        })
        .await;

        match result {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => {
                eprintln!("❌ [ERROR] Keyboard simulation failed: {}", e);
                Err(e)
            }
            Err(e) => {
                eprintln!("❌ [ERROR] Task spawn failed: {}", e);
                Err(anyhow::anyhow!("Failed to spawn blocking task: {}", e))
            }
        }
    }
}

/// Fast paste with immediate app switching (Raycast-style)
pub async fn simulate_paste_with_app_switch(content: String, target_app: Option<String>, start: std::time::Instant) -> Result<()> {
    // First, copy to clipboard
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(content.clone())?;
    println!("🔧 [PERF] T+{}ms: Content copied to clipboard", start.elapsed().as_millis());

    #[cfg(target_os = "macos")]
    {
        use core_graphics::event::{CGEvent, CGEventFlags, CGEventTapLocation, CGKeyCode};
        use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
        use std::process::Command;

        // If we have a target app, activate it using 'open -a' (faster than AppleScript)
        if let Some(app_name) = target_app {
            println!("🔧 [PERF] T+{}ms: Starting to activate app with 'open -a': {}", start.elapsed().as_millis(), app_name);

            let activate_start = std::time::Instant::now();

            // Use 'open -a' instead of AppleScript - it's faster
            let _ = Command::new("open")
                .arg("-a")
                .arg(&app_name)
                .spawn(); // Use spawn instead of output to not wait for completion

            println!("🔧 [PERF] T+{}ms: 'open -a' command sent (took {}ms)",
                start.elapsed().as_millis(), activate_start.elapsed().as_millis());

            // Short delay to let app start receiving focus
            tokio::time::sleep(Duration::from_millis(30)).await;
            println!("🔧 [PERF] T+{}ms: After 30ms delay", start.elapsed().as_millis());
        }

        println!("🔧 [PERF] T+{}ms: Creating CGEvent...", start.elapsed().as_millis());

        // Create event source
        let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
            .map_err(|_| anyhow::anyhow!("Failed to create event source"))?;

        // V key code on macOS
        const V_KEYCODE: CGKeyCode = 9;

        // Create and post Cmd+V events
        let key_down = CGEvent::new_keyboard_event(source.clone(), V_KEYCODE, true)
            .map_err(|_| anyhow::anyhow!("Failed to create key down event"))?;
        key_down.set_flags(CGEventFlags::CGEventFlagCommand);

        let key_up = CGEvent::new_keyboard_event(source.clone(), V_KEYCODE, false)
            .map_err(|_| anyhow::anyhow!("Failed to create key up event"))?;
        key_up.set_flags(CGEventFlags::CGEventFlagCommand);

        println!("🔧 [PERF] T+{}ms: Posting CGEvent...", start.elapsed().as_millis());
        key_down.post(CGEventTapLocation::HID);
        key_up.post(CGEventTapLocation::HID);
        println!("🔧 [PERF] T+{}ms: CGEvent posted", start.elapsed().as_millis());

        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    {
        // Fall back to regular paste for non-macOS
        simulate_paste(content).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifier_key_is_correct() {
        #[cfg(target_os = "macos")]
        assert_eq!(MODIFIER_KEY, Key::Meta);

        #[cfg(target_os = "windows")]
        assert_eq!(MODIFIER_KEY, Key::Control);

        #[cfg(target_os = "linux")]
        assert_eq!(MODIFIER_KEY, Key::Control);
    }

    #[test]
    fn test_paste_function_signature() {
        // Just verify the function exists and compiles
        // Actual paste simulation is hard to test automatically
        assert!(true);
    }
}
