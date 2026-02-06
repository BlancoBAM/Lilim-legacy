#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    GlobalShortcutManager,
};

fn main() {
    // Build system tray menu
    let show = CustomMenuItem::new("show".to_string(), "Show Chat");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide Chat");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit Lilim");
    
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            match event {
                SystemTrayEvent::LeftClick { .. } => {
                    let window = app.get_window("main").unwrap();
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    match id.as_str() {
                        "show" => {
                            let window = app.get_window("main").unwrap();
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                        "hide" => {
                            let window = app.get_window("main").unwrap();
                            window.hide().unwrap();
                        }
                        "quit" => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .setup(|app| {
            // Register global hotkey Ctrl+L
            let handle = app.handle();
            let mut shortcut_manager = app.global_shortcut_manager();
            
            shortcut_manager.register("Ctrl+L", move || {
                if let Some(window) = handle.get_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }).expect("Failed to register global shortcut Ctrl+L");

            println!("Lilim system tray started. Press Ctrl+L to toggle window.");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
