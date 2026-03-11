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

/// OI server base URL (configurable via LILIM_PORT env)
const OI_SERVER: &str = "http://localhost:8000";

#[tauri::command]
async fn send_query(query: String) -> Result<String, String> {
    // Send query to Open Interpreter server
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/chat", OI_SERVER))
        .json(&serde_json::json!({
            "message": query,
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to send query: {}", e))?;

    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(body)
}

#[tauri::command]
async fn generate_tts(text: String) -> Result<Vec<u8>, String> {
    // Generate TTS using lilith-tts binary (standalone)
    let output = tokio::process::Command::new("lilith-tts")
        .args(["speak", &text, "--output", "/tmp/lilim-tts-output.wav"])
        .output()
        .await
        .map_err(|e| format!("Failed to run lilith-tts: {}", e))?;

    if !output.status.success() {
        return Err(format!("TTS failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let audio_bytes = tokio::fs::read("/tmp/lilim-tts-output.wav")
        .await
        .map_err(|e| format!("Failed to read TTS output: {}", e))?;

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

    // Ctrl+Shift+L - Toggle Lilim window (global hotkey)
    let toggle_shortcut = Shortcut::new(
        Some(Modifiers::CONTROL | Modifiers::SHIFT),
        Code::KeyL,
    );
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

    // Ctrl+Shift+T - Read highlighted text / clipboard aloud via TTS
    // (Maps to the Ctrl+TTM concept — Ctrl+Shift+T is the ergonomic equivalent)
    let tts_shortcut = Shortcut::new(
        Some(Modifiers::CONTROL | Modifiers::SHIFT),
        Code::KeyT,
    );
    shortcut_plugin.on_shortcut(tts_shortcut, {
        move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                // Grab highlighted text (X11 primary selection) then fallback to clipboard
                tauri::async_runtime::spawn(async move {
                    // Try X11 primary selection first (highlighted text)
                    let text = tokio::process::Command::new("xsel")
                        .args(["--primary", "--output"])
                        .output()
                        .await
                        .ok()
                        .and_then(|o| {
                            if o.status.success() {
                                let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                                if !s.is_empty() { Some(s) } else { None }
                            } else { None }
                        });

                    // Fallback to clipboard
                    let text = match text {
                        Some(t) => t,
                        None => {
                            tokio::process::Command::new("xclip")
                                .args(["-selection", "clipboard", "-o"])
                                .output()
                                .await
                                .ok()
                                .and_then(|o| {
                                    if o.status.success() {
                                        let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                                        if !s.is_empty() { Some(s) } else { None }
                                    } else { None }
                                })
                                .unwrap_or_default()
                        }
                    };

                    if text.is_empty() {
                        eprintln!("[Lilim TTS] No text selected or in clipboard");
                        return;
                    }

                    eprintln!("[Lilim TTS] Reading aloud: {}...", &text[..text.len().min(50)]);

                    // Synthesize and play
                    let output_path = "/tmp/lilim-read-aloud.wav";
                    let result = tokio::process::Command::new("lilith-tts")
                        .args(["speak", &text, "--output", output_path])
                        .output()
                        .await;

                    match result {
                        Ok(o) if o.status.success() => {
                            // Play audio
                            let _ = tokio::process::Command::new("aplay")
                                .args([output_path])
                                .output()
                                .await;
                        }
                        Ok(o) => {
                            eprintln!("[Lilim TTS] Synthesis failed: {}", String::from_utf8_lossy(&o.stderr));
                        }
                        Err(e) => {
                            eprintln!("[Lilim TTS] Failed to run lilith-tts: {}", e);
                        }
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
