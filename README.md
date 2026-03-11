<p align="center">
  <img src="assets/lilith-icon.png" alt="Lilim" width="120" />
</p>

<h1 align="center">Lilim</h1>

<p align="center">
  <strong>Infernal AI Assistant for Lilith Linux</strong><br/>
  Sarcastic. Capable. Yours.
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#architecture">Architecture</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#configuration">Configuration</a> •
  <a href="#iphone-access">iPhone Access</a> •
  <a href="#contributing">Contributing</a>
</p>

---

## Features

🔥 **Intelligent Chatbot** — Sarcastic but caring AI assistant with infernal personality, powered by LiteLLM (GPT-4o, Claude, Ollama, and more)

⌨️ **Global Hotkey** — `Ctrl+Shift+L` summons Lilim from anywhere on your desktop

💻 **Code Execution** — Safely runs Python, JavaScript, and shell commands with user confirmation

📅 **Task Scheduling** — Schedule one-time and recurring tasks via natural language

🌐 **Browser Control** — Sandboxed web browsing for research and troubleshooting

📱 **iPhone Access** — Control your desktop from your iPhone via secure Gateway API with pairing authentication

🎙️ **Voice Synthesis** — Standalone TTS with NeuTTS nano (see [Lilith-TTS](https://github.com/BlancoBAM/Lilith-TTS))

🛡️ **Security First** — Supervised autonomy, sandboxed execution, forbidden path enforcement, pairing-based auth

## Architecture

```
 Ctrl+Shift+L
     │
 ┌───▼──────────┐    HTTP/SSE     ┌──────────────────────────┐
 │ Tauri UI      │ ◄────────────► │   Open Interpreter :8000 │
 │ (React flame) │                │   (Python brain)         │
 └──────────────┘                │   • Lilim personality    │
                                  │   • Code execution       │
 ┌──────────────┐  HTTPS/WSS     │   • LiteLLM routing      │
 │ iPhone       │ ◄──────────►  ├──────────────────────────┤
 │ (Shortcuts)  │  :42617        │   ZeroClaw Runtime       │
 └──────────────┘                │   • Gateway + pairing    │
                                  │   • Cron scheduler       │
                                  │   • Browser control      │
                                  │   • Memory (SQLite)      │
                                  └──────────────────────────┘
```

| Component | Tech | Purpose |
|-----------|------|---------|
| **Brain** | Python / Open Interpreter / LiteLLM | LLM routing, code execution, personality |
| **Runtime** | Rust / ZeroClaw | Security, scheduling, gateway, memory |
| **Desktop UI** | TypeScript / React / Tauri | Flame-themed chat interface |
| **TTS** | Rust / NeuTTS | Standalone voice synthesis ([separate repo](https://github.com/BlancoBAM/Lilith-TTS)) |

## Installation

### Prerequisites

- Linux (Ubuntu 22.04+ / Lilith Linux)
- Python 3.10+
- Rust 1.75+ (for ZeroClaw build)
- Node.js 18+ (for Tauri UI)
- `espeak-ng`, `cmake` (for TTS)

### Quick Install

```bash
# Clone
git clone https://github.com/BlancoBAM/Lilim.git
cd Lilim

# Install system dependencies
sudo apt install cmake espeak-ng python3-pip nodejs npm

# Install Open Interpreter
cd ../Lilim-v2 && pip install -e . && cd ../Lilim

# Build ZeroClaw runtime
git clone https://github.com/zeroclaw-labs/zeroclaw.git zeroclaw
cd zeroclaw && cargo build --release && cd ..
sudo cp zeroclaw/target/release/zeroclaw /usr/bin/

# Deploy configs
sudo mkdir -p /etc/lilith
sudo cp config/zeroclaw.toml /etc/lilith/
sudo cp config/lilim-identity.json /etc/lilith/

# Install service
sudo cp scripts/lilim-serve /usr/bin/
sudo chmod +x /usr/bin/lilim-serve
sudo cp systemd/system/lilith-ai.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now lilith-ai
```

### Desktop App

```bash
cd lilim_desktop
npm install
npm run tauri dev
```

## Usage

### Chat via Desktop

Press **`Ctrl+Shift+L`** to toggle the Lilim window. Type your message and press Enter.

### Example Conversations

```
You: What's my disk usage?
Lilim: Let me check that for you...
       > df -h
       Your root partition is at 67% — plenty of room. No fires to put out... yet.

You: Remind me to take a break in 30 minutes
Lilim: Done. I'll bug you in 30 minutes. Don't blame me when you're startled.
       > zeroclaw cron add-at "..." "Time for a break!"

You: Help me study anatomy terms
Lilim: *Cracks knuckles like a judgmental tutor*
       Alright, let's quiz you. What's the difference between the axial
       and appendicular skeleton?
```

### Read Aloud (TTS)

Hold `Ctrl` and press `T`, `T`, `M` in rapid succession to read highlighted text or clipboard contents aloud using Lilith-TTS.

## Configuration

### LLM Provider

Edit the Lilim profile at `/etc/lilith/lilim-profile.py`:

```python
interpreter.llm.model = "gpt-4o-mini"           # OpenAI
# interpreter.llm.model = "ollama/qwen3:4b"     # Local via Ollama
# interpreter.llm.model = "anthropic/claude-3.5" # Anthropic
```

### Autonomy Level

Edit `/etc/lilith/zeroclaw.toml`:

```toml
[autonomy]
level = "supervised"  # "readonly", "supervised", "full"
```

### Hotkey

The global hotkey can be changed in the Tauri source at `lilim_desktop/src-tauri/src/lib.rs`.

## iPhone Access

See [docs/iphone-setup.md](docs/iphone-setup.md) for full instructions.

**Quick version:**
1. Enable tunnel in `zeroclaw.toml` (`tunnel.provider = "cloudflare"`)
2. Get pairing code from Lilim desktop app
3. Create iOS Shortcut that POSTs to the gateway

## Project Structure

```
Lilim/
├── assets/                    # Icons and images
├── config/                    # ZeroClaw + identity configs
│   ├── zeroclaw.toml
│   └── lilim-identity.json
├── desktop/                   # .desktop launcher entry
├── docs/                      # Documentation
├── lilim_desktop/             # Tauri desktop app (React + Rust)
│   ├── src/                   # React frontend
│   └── src-tauri/             # Tauri backend (Rust)
├── scripts/                   # Server launch scripts
├── systemd/                   # systemd service files
└── lilith-tts/                # TTS module (separate repo)
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

This project is licensed under **AGPL-3.0** (inherited from Open Interpreter and ZeroClaw).

See [LICENSE](LICENSE) for details.
