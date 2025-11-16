use tauri::{AppHandle, Emitter, Manager, WebviewWindow};
use anyhow::Result;
use std::sync::Mutex;

// Store the previously active application name
static PREVIOUS_APP: Mutex<Option<String>> = Mutex::new(None);

pub fn show_window(app: &AppHandle) -> Result<()> {
    // Before showing SuperKBD, get the currently frontmost application
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        println!("🔧 [DEBUG] Getting frontmost app before showing SuperKBD...");
        let result = Command::new("osascript")
            .arg("-e")
            .arg(r#"tell application "System Events"
    set frontApp to first application process whose frontmost is true
    return name of frontApp
end tell"#)
            .output();

        if let Ok(output) = result {
            if output.status.success() {
                let app_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !app_name.is_empty() && app_name != "SuperKBD" && app_name != "superkbd" {
                    println!("🔧 [DEBUG] Storing previous app: {}", app_name);
                    if let Ok(mut prev) = PREVIOUS_APP.lock() {
                        *prev = Some(app_name);
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

        // On macOS, activate the previously stored application
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;

            // Get the stored previous app name
            let previous_app = if let Ok(prev) = PREVIOUS_APP.lock() {
                prev.clone()
            } else {
                None
            };

            if let Some(app_name) = previous_app {
                if !app_name.is_empty() {
                    println!("🔧 [DEBUG] Activating stored previous app: {}", app_name);
                    let activate_script = format!(r#"tell application "{}" to activate"#, app_name);
                    let result = Command::new("osascript")
                        .arg("-e")
                        .arg(&activate_script)
                        .output();

                    if let Ok(output) = result {
                        if output.status.success() {
                            println!("✅ [DEBUG] Successfully activated: {}", app_name);
                        } else {
                            eprintln!("❌ [ERROR] Failed to activate {}: {}", app_name, String::from_utf8_lossy(&output.stderr));
                        }
                    }
                } else {
                    println!("⚠️  [WARNING] Previous app name is empty");
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
