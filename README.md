# 🔥 Lilim AI Assistant

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-Linux-blue.svg)](https://www.linux.org/)

**Lilim** is an intelligent multi-model AI assistant system designed for Lilith Linux, featuring advanced routing, local-first processing, and seamless desktop integration.

---

## ✨ Features

### 🤖 Intelligent Multi-Model Routing
- **Router Model** (Qwen2-0.5B): Fast query classification and model selection
- **Main Agentic Model** (Qwen3-VL 4B): Local reasoning with vision capabilities
- **TTS Model** (neutts-nano): Offline voice synthesis

### 🌐 Flexible Provider System
- Configure unlimited API providers (OpenAI, Anthropic, custom endpoints)
- Automatic fallback to online providers for complex queries
- Environment-based API key management
- Support for multiple authentication methods

### 🖥️ Desktop Integration
- **System Tray**: Always-visible icon with quick access menu
- **Global Hotkeys**:
  - `Ctrl+L`: Toggle Lilim window
  - `Ctrl+Shift+M`: TTS screen reader
- **Native Window**: Tauri-based COSMIC desktop application
- **Auto-start**: Launches with system

### 🔊 Screen Reading & TTS
- Capture browser content (full HTML, even off-screen)
- Capture terminal output (prompt → stdout)
- Text-to-speech synthesis with neutts-nano
- Accessibility API integration

### 🧠 RAG Knowledge Base
- **Vector Search**: fastembed for semantic retrieval
- **Keyword Search**: tantivy for full-text search
- Pre-loaded domains:
  - 📚 Linux/Ubuntu troubleshooting
  - 🏥 Medical terminology & procedures
  - 🎓 Academic study techniques

### 🛠️ Agentic Tool Use
- **Web Search**: Real-time information retrieval
- **File Search**: Local filesystem operations
- **Terminal**: Command execution (safe mode)
- **YOLO Mode**: Optional auto-execution

### 💾 Conversation Memory
- SQLite-based chat history
- Session management
- Context window optimization

---

## 📦 Installation

### Method 1: .deb Package (Recommended)

```bash
# Download and install
wget https://github.com/BlancoBAM/Lilim/releases/latest/download/lilim-ai_0.1.0_amd64.deb
sudo dpkg -i lilim-ai_0.1.0_amd64.deb
sudo apt-get install -f  # Fix dependencies
```

### Method 2: From Source

```bash
# Clone repository
git clone https://github.com/BlancoBAM/Lilim.git
cd Lilim

# Install system dependencies
sudo apt-get install -y \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# Build and install
sudo ./install.sh
```

### System Requirements

- **OS**: Ubuntu 24.04+ or compatible Linux distribution
- **RAM**: 16GB+ recommended (8GB minimum)
- **Disk**: 10GB for models and data
- **CPU**: Modern x86_64 with AVX2 support
- **GPU**: Optional (reduces local model latency)

---

## 🚀 Usage

### Desktop Application

1. **Launch**: Press `Ctrl+L` or click the system tray icon
2. **Ask Questions**: Type naturally in the chat interface
3. **TTS Screen Reader**: Press `Ctrl+Shift+M` to read screen content

### CLI (Optional)

```bash
# Ask a question
lilim ask "How do I restart a systemd service?"

# Search knowledge base
lilim search "cardiovascular system"

# View conversation history
lilim history
```

---

## ⚙️ Configuration

Edit `/etc/lilith/lilim.yaml`:

```yaml
# Local Models
models:
  local:
    router:
      model_path: "/var/lib/lilith/models/qwen2-0.5b.gguf"
    main:
      model_path: "/var/lib/lilith/models/qwen3-vl-4b.gguf"
    tts:
      model_path: "/var/lib/lilith/models/neutts-nano-q4.gguf"

# Online Providers
providers:
  online:
    - name: "openai"
      api_url: "https://api.openai.com/v1/chat/completions"
      model: "gpt-4o-mini"
      
# Routing
routing:
  prefer_local: true
  max_local_latency_ms: 5000

# Hotkeys
system:
  hotkeys:
    toggle_ui: "Ctrl+L"
    tts_screen_reader: "Ctrl+Shift+M"
```

---

## 📚 Documentation

- [**User Manual**](docs/USER_MANUAL.md) - Complete usage guide
- [**Installation Guide**](docs/INSTALL_GUIDE.md) - Detailed setup instructions
- [**API Reference**](docs/API_REFERENCE.md) - HTTP API documentation

---

## 🗂️ Project Structure

```
Lilim/
├── lilim_core/           # Core functionality (RAG, config, API)
├── lilim_server/         # HTTP API server
├── lilim_desktop/        # Tauri desktop application
├── lilim_tts/           # TTS engine (neutts-nano)
├── lilim_cli/           # CLI tool
├── lilith_rag_builder/  # RAG index builder
├── lilith_search/       # RAG search utility
├── config/              # Configuration templates
├── packaging/           # .deb packaging scripts
├── docs/                # Documentation
└── install.sh           # Installation script
```

---

## 📄 License

Lilim is dual-licensed under MIT License and Apache License 2.0.

---

<p align="center">
  <strong>Built with 🔥 for Lilith Linux</strong>
</p>
