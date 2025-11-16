use crate::database::{self, ClipboardEntry, ClipboardSearchParams};
use crate::paste;
use crate::window;
use anyhow::Result;
use sqlx::SqlitePool;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_updater::UpdaterExt;

pub struct AppState {
    pub pool: Arc<SqlitePool>,
}

#[tauri::command]
pub async fn get_clipboard_history(
    search_query: Option<String>,
    limit: Option<i64>,
    favorites_only: Option<bool>,
    state: State<'_, AppState>,
) -> Result<Vec<ClipboardEntry>, String> {
    let params = ClipboardSearchParams {
        query: search_query,
        favorites_only: favorites_only.unwrap_or(false),
        limit: limit.unwrap_or(100),
        ..Default::default()
    };

    database::search_entries(&state.pool, params)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_favorite(
    id: i64,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    database::toggle_favorite(&state.pool, id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_clipboard_entry(
    id: i64,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    database::delete_entry(&state.pool, id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn paste_and_close(
    content: String,
    app: AppHandle,
    _state: State<'_, AppState>,
) -> Result<(), String> {
    use std::time::Instant;

    let start = Instant::now();
    println!("\n⏱️  [PERF] ========== PASTE START ==========");
    println!("🚀 [PERF] T+0ms: paste_and_close called");

    // Get the target app name before hiding
    let target_app = window::get_previous_app();
    println!("🔧 [PERF] T+{}ms: Got target app: {:?}", start.elapsed().as_millis(), target_app);

    // Hide window
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| e.to_string())?;
    }
    println!("🔧 [PERF] T+{}ms: Window hidden", start.elapsed().as_millis());

    // Immediately paste - don't wait for natural focus switch
    paste::simulate_paste_with_app_switch(content, target_app, start)
        .await
        .map_err(|e| e.to_string())?;

    println!("✅ [PERF] T+{}ms: ========== PASTE COMPLETE ==========\n", start.elapsed().as_millis());
    Ok(())
}

#[tauri::command]
pub async fn cleanup_old_entries(
    days: i64,
    state: State<'_, AppState>,
) -> Result<u64, String> {
    database::delete_old_entries(&state.pool, days)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn hide_window_command(app: AppHandle) -> Result<(), String> {
    window::hide_window(&app).map_err(|e| e.to_string())
}

/// Test command: Only copy to clipboard without simulating paste
/// This is useful for testing without requiring accessibility permissions
#[tauri::command]
pub async fn copy_to_clipboard_only(
    content: String,
    app: AppHandle,
) -> Result<(), String> {
    // Hide window first
    window::hide_window(&app).map_err(|e| e.to_string())?;

    // Small delay to let window hide
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Only copy to clipboard, don't simulate paste
    let mut clipboard = arboard::Clipboard::new()
        .map_err(|e| format!("Failed to access clipboard: {}", e))?;
    clipboard.set_text(content)
        .map_err(|e| format!("Failed to set clipboard: {}", e))?;

    Ok(())
}

/// Check if we have accessibility permissions for auto-paste
#[tauri::command]
pub fn check_accessibility_permission() -> bool {
    paste::check_accessibility_permission()
}

/// Open system preferences to grant accessibility permissions
#[tauri::command]
pub fn open_accessibility_settings() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .spawn()
            .map_err(|e| format!("Failed to open System Settings: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        // Windows doesn't require special permissions for keyboard simulation
    }

    Ok(())
}

/// Check for updates manually - can be called from menu
#[tauri::command]
pub async fn check_for_updates(app: AppHandle) -> Result<(), String> {
    check_for_updates_impl(&app).await.map_err(|e| e.to_string())
}

/// Implementation of update check that can be called from both command and menu
pub async fn check_for_updates_impl(app: &AppHandle) -> Result<()> {
    use tauri::async_runtime::spawn;

    println!("🔍 Checking for updates...");

    let updater = app.updater_builder().build()?;

    match updater.check().await {
        Ok(Some(update)) => {
            let version = update.version.clone();
            let current_version = update.current_version.clone();
            let body = update.body.clone().unwrap_or_else(|| "No release notes available".to_string());

            println!("✅ Update available: {}", version);
            println!("📝 Current version: {}", current_version);
            println!("📄 Release notes: {}", body);

            // Show confirmation dialog using rfd (native dialog)
            let app_handle = app.clone();
            spawn(async move {
                use rfd::MessageButtons;
                use rfd::MessageLevel;

                let message = format!(
                    "A new version {} is available (you have {}).\n\nThe update will be downloaded and installed. Would you like to continue?",
                    version, current_version
                );

                let response = rfd::MessageDialog::new()
                    .set_title("Update Available")
                    .set_description(&message)
                    .set_level(MessageLevel::Info)
                    .set_buttons(MessageButtons::OkCancel)
                    .show();

                if response == rfd::MessageDialogResult::Ok {
                    println!("🔄 User confirmed - Downloading and installing update...");
                    let mut downloaded = 0;

                    match update.download_and_install(
                        |chunk_length, content_length| {
                            downloaded += chunk_length;
                            if let Some(total) = content_length {
                                let progress = (downloaded as f64 / total as f64) * 100.0;
                                if progress as i32 % 10 == 0 {
                                    println!("📥 Download progress: {:.0}%", progress);
                                }
                            }
                        },
                        || {
                            println!("✅ Download completed, installing...");
                        }
                    ).await {
                        Ok(_) => {
                            println!("🎉 Update installed successfully!");

                            rfd::MessageDialog::new()
                                .set_title("Update Installed")
                                .set_description("Update installed successfully! Please restart the application to use the new version.")
                                .set_level(MessageLevel::Info)
                                .set_buttons(MessageButtons::Ok)
                                .show();
                        }
                        Err(e) => {
                            eprintln!("❌ Installation failed: {}", e);

                            rfd::MessageDialog::new()
                                .set_title("Update Failed")
                                .set_description(&format!("Failed to install update: {}", e))
                                .set_level(MessageLevel::Error)
                                .set_buttons(MessageButtons::Ok)
                                .show();
                        }
                    }
                } else {
                    println!("❌ User cancelled update");
                }
            });
        }
        Ok(None) => {
            println!("✅ No updates available - already on latest version");

            // Show info message using rfd
            rfd::MessageDialog::new()
                .set_title("No Updates")
                .set_description("You are already running the latest version.")
                .set_level(rfd::MessageLevel::Info)
                .set_buttons(rfd::MessageButtons::Ok)
                .show();
        }
        Err(e) => {
            eprintln!("❌ Update check failed: {}", e);

            // Show error message using rfd
            let error_msg = format!("Failed to check for updates: {}", e);
            rfd::MessageDialog::new()
                .set_title("Update Check Failed")
                .set_description(&error_msg)
                .set_level(rfd::MessageLevel::Error)
                .set_buttons(rfd::MessageButtons::Ok)
                .show();

            return Err(e.into());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commands_exist() {
        // Verify all commands compile
        assert!(true);
    }
}
