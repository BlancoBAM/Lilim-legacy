// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime,
};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

mod commands;
mod screen_reader;

#[tauri::command]
async fn send_query(query: String) -> Result<String, String> {
    // Send query to lilim_server API
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:8080/chat")
        .json(&serde_json::json!({
            "query": query,
            "session_id": generate_session_id(),
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to send query: {}", e))?;

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(result["response"].as_str().unwrap_or("").to_string())
}

#[tauri::command]
async fn generate_tts(text: String) -> Result<Vec<u8>, String> {
    // Call TTS endpoint
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:8080/tts")
        .json(&serde_json::json!({
            "text": text,
            "format": "wav",
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to generate TTS: {}", e))?;

    let audio_bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read audio: {}", e))?
        .to_vec();

    Ok(audio_bytes)
}

#[tauri::command]
async fn capture_screen_content() -> Result<String, String> {
    // Call screen reader module
    screen_reader::capture_active_window()
        .await
        .map_err(|e| format!("Failed to capture screen: {}", e))
}

fn generate_session_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("session_{}", timestamp)
}

fn setup_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let show_i = MenuItem::with_id(app, "show", "Show Lilim", true, None::<&str>)?;
    let hide_i = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show_i, &hide_i, &quit_i])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(true)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}

fn setup_global_shortcuts<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let shortcut_plugin = app.state::<tauri_plugin_global_shortcut::GlobalShortcut>();

    // Ctrl+L - Toggle window visibility
    let toggle_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyL);
    shortcut_plugin.on_shortcut(toggle_shortcut, {
        let app_handle = app.clone();
        move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                if let Some(window) = app_handle.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        }
    })?;

    // Ctrl+Shift+M - TTS Screen Reader
    let tts_shortcut = Shortcut::new(
        Some(Modifiers::CONTROL | Modifiers::SHIFT),
        Code::KeyM,
    );
    shortcut_plugin.on_shortcut(tts_shortcut, {
        let app_handle = app.clone();
        move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                // Trigger screen capture and TTS
                tauri::async_runtime::spawn(async move {
                    if let Ok(content) = screen_reader::capture_active_window().await {
                        // Send content for TTS
                        let client = reqwest::Client::new();
                        let _ = client
                            .post("http://localhost:8080/tts")
                            .json(&serde_json::json!({
                                "text": content,
                                "format": "wav",
                            }))
                            .send()
                            .await;
                    }
                });
            }
        }
    })?;

    shortcut_plugin.register(toggle_shortcut)?;
    shortcut_plugin.register(tts_shortcut)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            send_query,
            generate_tts,
            capture_screen_content
        ])
        .setup(|app| {
            setup_tray(app.handle())?;
            setup_global_shortcuts(app.handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
