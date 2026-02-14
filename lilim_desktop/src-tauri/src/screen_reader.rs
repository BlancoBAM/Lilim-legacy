// Screen reading capabilities for browser and terminal content
use anyhow::{anyhow, Result};
use std::process::Command;

pub async fn capture_active_window() -> Result<String> {
    // Get active window class/name to determine if it's browser or terminal
    let window_info = get_active_window_info()?;
    
    if is_browser(&window_info) {
        capture_browser_content().await
    } else if is_terminal(&window_info) {
        capture_terminal_content().await
    } else {
        // Fallback: try to get clipboard or selected text
        capture_clipboard().await
    }
}

fn get_active_window_info() -> Result<String> {
    // Use xdotool or similar to get active window
    let output = Command::new("xdotool")
        .args(&["getactivewindow", "getwindowname"])
        .output()?;
    
    String::from_utf8(output.stdout)
        .map_err(|e| anyhow!("Failed to parse window name: {}", e))
}

fn is_browser(window_info: &str) -> bool {
    let browsers = ["firefox", "chrome", "chromium", "brave",  "edge"];
    let lower = window_info.to_lowercase();
    browsers.iter().any(|b| lower.contains(b))
}

fn is_terminal(window_info: &str) -> bool {
    let terminals = ["terminal", "konsole", "gnome-terminal", "alacritty", "kitty", "cosmic-term"];
    let lower = window_info.to_lowercase();
    terminals.iter().any(|t| lower.contains(t))
}

async fn capture_browser_content() -> Result<String> {
    // Method 1: Try using accessibility APIs via AT-SPI
    if let Ok(content) = capture_via_atspi().await {
        return Ok(content);
    }
    
    // Method 2: Try getting full page via browser automation (if installed)
    if let Ok(content) = capture_via_browser_automation().await {
        return Ok(content);
    }
    
    // Method 3: Fallback to clipboard
    capture_clipboard().await
}

async fn capture_terminal_content() -> Result<String> {
    // Get the current TTY/terminal buffer
    // This is complex and depends on the terminal emulator
    
    // Method 1: Try reading from /proc
    if let Ok(content) = read_terminal_from_proc().await {
        return Ok(content);
    }
    
    // Method 2: Fallback to clipboard
    capture_clipboard().await
}

async fn capture_via_atspi() -> Result<String> {
    // Use AT-SPI (Assistive Technology Service Provider Interface)
    // This requires atspi crate or command-line tool
    
    let output = Command::new("accerciser")
        .args(&["--console", "--dump-tree"])
        .output()?;
    
    let content = String::from_utf8(output.stdout)?;
    
    // Parse and extract text content
    extract_text_from_atspi(&content)
}

async fn capture_via_browser_automation() -> Result<String> {
    // This would use headless browser or browser extensions
    // For now, return error to fall back
    Err(anyhow!("Browser automation not implemented"))
}

async fn read_terminal_from_proc() -> Result<String> {
    // Try to read terminal buffer from /proc
    // This is terminal-specific and may not work on all terminals
    Err(anyhow!("Terminal proc reading not implemented"))
}

async fn capture_clipboard() -> Result<String> {
    // Fallback: read from clipboard using wl-paste or xclip
    let output = if is_wayland() {
        Command::new("wl-paste").output()
    } else {
        Command::new("xclip")
            .args(&["-selection", "clipboard", "-o"])
            .output()
    };
    
    match output {
        Ok(out) => String::from_utf8(out.stdout)
            .map_err(|e| anyhow!("Failed to parse clipboard: {}", e)),
        Err(e) => Err(anyhow!("Failed to read clipboard: {}", e)),
    }
}

fn is_wayland() -> bool {
    std::env::var("WAYLAND_DISPLAY").is_ok()
}

fn extract_text_from_atspi(atspi_output: &str) -> Result<String> {
    // Parse AT-SPI tree output and extract text content
    // This is a simplified implementation
    let mut text = String::new();
    
    for line in atspi_output.lines() {
        if line.contains("text:") {
            if let Some(content) = line.split("text:").nth(1) {
                text.push_str(content.trim());
                text.push(' ');
            }
        }
    }
    
    if text.is_empty() {
        Err(anyhow!("No text content found in AT-SPI output"))
    } else {
        Ok(text)
    }
}

// Helper: Capture specific browser tab content (requires browser extension)
pub async fn capture_browser_tab(url: &str) -> Result<String> {
    // This would integrate with a browser extension that can export page content
    // For MVP, we'll use simpler methods
    Err(anyhow!("Browser extension not implemented"))
}
