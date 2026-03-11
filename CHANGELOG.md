# Changelog

All notable changes to Lilim will be documented in this file.

## [0.2.0] — 2026-03-10

### Added
- **ZeroClaw Runtime Integration** — Rust backbone for security, scheduling, gateway, and memory
- **Global Hotkey** — `Ctrl+Shift+L` toggles Lilim window from anywhere
- **iPhone Access** — Secure Gateway API with pairing-code authentication
- **Task Scheduling** — `zeroclaw cron` for one-time and recurring tasks
- **Browser Control** — Sandboxed web browsing via ZeroClaw
- **Animated Panel Icon** — Pulsing glow animation on hover/loading
- **TTS Panel Icon** — Voice selection, speed adjustment, voice creation workflow
- **Ctrl+TTM Read Aloud** — Read highlighted text or clipboard contents via TTS
- **Voice Management** — Create, preview, and save custom voices from audio samples
- **AIEOS Identity** — Portable persona spec for Lilim personality

### Changed
- Backend migrated from custom Rust server to **Open Interpreter** (Python/LiteLLM)
- TTS separated into standalone `lilith-tts` crate (see [Lilith-TTS](https://github.com/BlancoBAM/Lilith-TTS))
- API endpoints updated from `:8080` to `:8000` (OI server)
- Tauri config fixed (product name, identifier, CSP)
- Systemd service updated for dual-process architecture

## [0.1.0] — 2026-03-01

### Added
- Initial Lilim project structure
- Tauri desktop app with flame theme
- Lilim personality system (responses YAML)
- Basic chat interface
