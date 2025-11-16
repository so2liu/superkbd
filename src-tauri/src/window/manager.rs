use tauri::{AppHandle, Emitter, Manager, WebviewWindow};
use anyhow::Result;
use std::sync::Mutex;

// Store the previously active application name
static PREVIOUS_APP: Mutex<Option<String>> = Mutex::new(None);

pub fn show_window(app: &AppHandle) -> Result<()> {
    // Before showing SuperKBD, get the currently frontmost application's bundle ID
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        println!("🔧 [DEBUG] Getting frontmost app before showing SuperKBD...");
        let result = Command::new("osascript")
            .arg("-e")
            .arg(r#"tell application "System Events"
    set frontApp to first application process whose frontmost is true
    return bundle identifier of frontApp
end tell"#)
            .output();

        if let Ok(output) = result {
            if output.status.success() {
                let bundle_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !bundle_id.is_empty() && bundle_id != "com.so2liu.superkbd" {
                    println!("🔧 [DEBUG] Storing previous app bundle ID: {}", bundle_id);
                    if let Ok(mut prev) = PREVIOUS_APP.lock() {
                        *prev = Some(bundle_id);
                    }
                }
            }
        }
    }

    if let Some(window) = app.get_webview_window("main") {
        window.show()?;
        window.set_focus()?;
        window.center()?;

        // Emit event to frontend for focus
        let _ = app.emit("window-shown", ());
    }
    Ok(())
}

pub fn hide_window(app: &AppHandle) -> Result<()> {
    if let Some(window) = app.get_webview_window("main") {
        // First hide the window
        window.hide()?;

        // On macOS, activate the previously stored application using bundle ID
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;

            // Get the stored previous app bundle ID
            let previous_bundle_id = if let Ok(prev) = PREVIOUS_APP.lock() {
                prev.clone()
            } else {
                None
            };

            if let Some(bundle_id) = previous_bundle_id {
                if !bundle_id.is_empty() {
                    println!("🔧 [DEBUG] Activating app with bundle ID: {}", bundle_id);
                    // Use 'open -b' to open by bundle identifier - more reliable than app name
                    let result = Command::new("open")
                        .arg("-b")
                        .arg(&bundle_id)
                        .output();

                    if let Ok(output) = result {
                        if output.status.success() {
                            println!("✅ [DEBUG] Successfully activated bundle: {}", bundle_id);
                        } else {
                            eprintln!("❌ [ERROR] Failed to activate {}: {}", bundle_id, String::from_utf8_lossy(&output.stderr));
                        }
                    }
                } else {
                    println!("⚠️  [WARNING] Previous app bundle ID is empty");
                }
            } else {
                println!("⚠️  [WARNING] No previous app stored");
            }
        }
    }
    Ok(())
}

pub fn get_previous_app() -> Option<String> {
    if let Ok(prev) = PREVIOUS_APP.lock() {
        prev.clone()
    } else {
        None
    }
}

pub fn toggle_window(app: &AppHandle) -> Result<()> {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible()? {
            println!("🔧 [DEBUG] Window is visible - hiding");
            hide_window(app)?;
        } else {
            println!("🔧 [DEBUG] Window is hidden - showing");
            show_window(app)?;
        }
    }
    Ok(())
}

pub fn setup_window_events(window: &WebviewWindow) {
    let window_clone = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            // Prevent close, hide instead
            let _ = window_clone.hide();
            api.prevent_close();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_functions_exist() {
        // These are mainly integration tests
        // Just verify the functions compile
        assert!(true);
    }
}
