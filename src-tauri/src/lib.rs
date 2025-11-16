mod clipboard;
mod commands;
mod database;
mod paste;
mod window;

use clipboard::ClipboardMonitor;
use commands::AppState;
use database::Database;
use std::sync::Arc;
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState};
use tauri::menu::{Menu, MenuItem};
use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Get app data directory
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            // Initialize database
            let db_path = app_data_dir.join("clipboard.db");
            let db = tauri::async_runtime::block_on(async {
                Database::new(db_path).await.expect("Failed to initialize database")
            });

            let pool = Arc::new(db.pool().clone());

            // Store database pool in app state
            app.manage(AppState {
                pool: Arc::clone(&pool),
            });

            // Start clipboard monitoring
            let monitor = ClipboardMonitor::new(Arc::clone(&pool));
            monitor.start(app.handle().clone());

            // Check accessibility permissions on startup
            #[cfg(target_os = "macos")]
            {
                let app_handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    // Wait a bit for the window to be ready
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

                    if !paste::check_accessibility_permission() {
                        println!("⚠️  Accessibility permissions not granted");
                        println!("   Auto-paste feature will not work until permissions are granted.");

                        // Emit event to frontend to show permission dialog
                        let _ = app_handle.emit("accessibility-permission-needed", ());
                    } else {
                        println!("✅ Accessibility permissions granted - auto-paste enabled");
                    }
                });
            }

            // Register global shortcut using the working pattern from demo
            let alt_i_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyI);
            let app_handle_for_shortcut = app.handle().clone();

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |_app, shortcut, _event| {
                        if shortcut == &alt_i_shortcut {
                            println!("Alt+I pressed - showing window");
                            let _ = window::show_window(&app_handle_for_shortcut);
                        }
                    })
                    .build(),
            )?;

            app.global_shortcut().register(alt_i_shortcut)?;

            // Setup window event handlers
            if let Some(window) = app.get_webview_window("main") {
                window::setup_window_events(&window);
            }

            // Setup system tray
            let show_item = MenuItem::with_id(app, "show", "Show Clipboard History", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| {
                    match event.id().as_ref() {
                        "show" => {
                            let _ = window::show_window(app);
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        let _ = window::show_window(app);
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_clipboard_history,
            commands::toggle_favorite,
            commands::delete_clipboard_entry,
            commands::paste_and_close,
            commands::copy_to_clipboard_only,
            commands::check_accessibility_permission,
            commands::open_accessibility_settings,
            commands::cleanup_old_entries,
            commands::hide_window_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
