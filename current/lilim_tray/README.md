# Lilim System Tray - Setup Instructions

## Prerequisites

1. **Install Tauri dependencies**:
   ```bash
   sudo apt-get install -y libwebkit2gtk-4.0-dev \
       libgtk-3-dev \
       libayatana-appindicator3-dev \
       librsvg2-dev
   ```

2. **Install Hugging Face CLI** (for model download):
   ```bash
   pipx install 'huggingface_hub[cli]'
   # OR
   pip3 install --break-system-packages 'huggingface_hub[cli]'
   ```

## Download Qwen3-VL Model

```bash
# Create models directory
mkdir -p /home/blanco/models

# Download Qwen3-VL-4B-Instruct (official model name)
huggingface-cli download Qwen/Qwen3-VL-4B-Instruct \
  --local-dir /home/blanco/models/Qwen3-VL-4B-Instruct

# Alternative: Download pre-quantized GGUF (smaller, faster)
huggingface-cli download bartowski/Qwen3-VL-4B-Instruct-GGUF \
  --include "*Q4_K_M.gguf" \
  --local-dir /home/blanco/models/Qwen3-VL-4B-GGUF
```

## Build and Run

### 1. Start the backend server
```bash
cd /home/blanco/Lilim/current/lilim_server
cargo run --release
```

### 2. Build and run the system tray app
```bash
cd /home/blanco/Lilim/current/lilim_tray
cargo build --release
./target/release/lilim_tray
```

## Usage

- **System Tray Icon**: Look for the flame icon in your system tray
- **Global Hotkey**: Press **Ctrl+L** to show/hide the chat window
- **Tray Menu**: Right-click the icon for Show/Hide/Quit options

## Next Steps

1. **Implement Crane Model Loading**: Update `lilim_server` to actually load and use Qwen3-VL via Crane
2. **Update UI API Calls**: Ensure chat UI connects to `http://localhost:8080/chat`
3. **Test Integration**: Send messages and verify responses come from local model
4. **Create Desktop Entry**: Add to system startup if desired
