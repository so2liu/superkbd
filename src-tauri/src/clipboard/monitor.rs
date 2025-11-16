use anyhow::Result;
use arboard::Clipboard;
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::RwLock;
use tokio::time::interval;

use super::types::ClipboardContent;
use crate::database::{self, NewClipboardEntry};

pub struct ClipboardMonitor {
    last_content: Arc<RwLock<Option<String>>>,
    pool: Arc<SqlitePool>,
}

impl ClipboardMonitor {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self {
            last_content: Arc::new(RwLock::new(None)),
            pool,
        }
    }

    pub fn start(&self, app_handle: AppHandle) {
        let last_content = Arc::clone(&self.last_content);
        let pool = Arc::clone(&self.pool);

        tauri::async_runtime::spawn(async move {
            let mut interval = interval(Duration::from_millis(500));
            let mut clipboard = Clipboard::new().expect("Failed to access clipboard");

            loop {
                interval.tick().await;

                // Try to get clipboard text
                if let Ok(text) = clipboard.get_text() {
                    let should_process = {
                        let last = last_content.read().await;
                        match last.as_ref() {
                            Some(last_text) => last_text != &text,
                            None => true,
                        }
                    };

                    if should_process && !text.trim().is_empty() {
                        // Update last content
                        {
                            let mut last = last_content.write().await;
                            *last = Some(text.clone());
                        }

                        // Create clipboard content
                        let content = ClipboardContent::new_text(text.clone());

                        // Emit event to frontend
                        let _ = app_handle.emit("clipboard-update", &content);

                        // Get the frontmost application name
                        let source_app = get_frontmost_app();

                        // Save to database (using upsert to avoid duplicates)
                        let entry = NewClipboardEntry::new_text_with_source(text, source_app);
                        if let Err(e) = database::upsert_entry(&pool, entry).await {
                            eprintln!("Failed to save clipboard entry: {}", e);
                        }
                    }
                }
            }
        });
    }
}

/// Get the frontmost application name on macOS
fn get_frontmost_app() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

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
                    return Some(app_name);
                }
            }
        }
    }

    None
}

pub fn should_save_content(content: &str, last_content: Option<&str>) -> bool {
    // Don't save empty content
    if content.trim().is_empty() {
        return false;
    }

    // Don't save if same as last
    if let Some(last) = last_content {
        if last == content {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_save_content() {
        // Empty content should not be saved
        assert!(!should_save_content("", None));
        assert!(!should_save_content("  ", None));

        // New content should be saved
        assert!(should_save_content("Hello", None));

        // Different content should be saved
        assert!(should_save_content("Hello", Some("World")));

        // Same content should not be saved
        assert!(!should_save_content("Hello", Some("Hello")));
    }

    #[test]
    fn test_clipboard_content_deduplication() {
        let content1 = "Hello World";
        let content2 = "Hello World";
        let content3 = "Different";

        assert!(!should_save_content(content2, Some(content1)));
        assert!(should_save_content(content3, Some(content1)));
    }
}
