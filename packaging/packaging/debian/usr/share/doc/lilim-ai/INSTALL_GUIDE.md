# Lilim Installation Guide

## Prerequisites

### System Requirements
- **OS**: Ubuntu 22.04+ or Lilith Linux
- **RAM**: Minimum 4GB (8GB recommended)
-**Disk**: 2GB free space
- **CPU**: 2+ cores recommended

### Required Software
- **Rust toolchain** (1.70+)
- **Node.js** (18+) and npm
- **Build essentials**

Install prerequisites:
```bash
# Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Node.js
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# Build tools
sudo apt install -y build-essential pkg-config libssl-dev
```

---

## Installation Steps

### 1. Clone or Extract Source

```bash
cd /path/to/Lilim
```

### 2. Run Installation Script

```bash
sudo ./install.sh
```

The installer will:
1. Build all Rust binaries (lilim_server, lilith_rag_builder, lilith_search, lilim CLI)
2. Create system user `lilith`
3. Set up directory structure in `/var/lib/lilith`, `/etc/lilith`, `/usr/bin`
4. Install systemd service
5. Install desktop launcher
6. Set appropriate permissions

**Installation time**: 5-15 minutes depending on system

### 3. Configure Lilim

Edit `/etc/lilith/lilim.yaml`:

```bash
sudo nano /etc/lilith/lilim.yaml
```

**Minimal configuration** (works without API keys):
```yaml
models:
  primary: "api-fallback"  # Use local RAG only
  fallback_provider: "none"

agent:
  enable_web_search: false  # Disable if no API keys
  enable_file_search: true
  enable_terminal: true
  yolo_mode: false
```

**With API integration** (optional):
```yaml
api_keys:
  openai: "sk-your-key-here"
  # OR
  anthropic: "sk-ant-your-key-here"
```

### 4. Build RAG Knowledge Base (if not pre-built)

```bash
cd /home/blanco/final-Lil/Lilim/lilith_rag_builder
sudo -u lilith /usr/bin/lilith_rag_builder
```

This creates vector and keyword indices from `rag_ready/rag_text.jsonl`.

### 5. Start Lilim Service

```bash
# Start immediately
sudo systemctl start lilith-ai

# Enable auto-start on boot
sudo systemctl enable lilith-ai

# Check status
sudo systemctl status lilith-ai
```

### 6. Verify Installation

**Test CLI**:
```bash
lilim status
lilim ask "Hello, Lilim!"
```

**Test Web UI**:
```bash
xdg-open http://localhost:8080
```

---

## Post-Installation

### Build Frontend UI (Optional - for development)

The frontend is pre-built during installation. To rebuild:

```bash
cd /home/blanco/LIL/LILIM/UI/chat-design
npm install
npm run build

# Copy build to installation directory
sudo cp -r build/* /usr/share/lilith/web/
```

### Desktop Integration

The desktop launcher should appear in your app menu:
- Press `Super` key
- Type "Lilim"  
- Click to launch

To manually copy:
```bash
cp desktop/lilith-ai.desktop ~/.local/share/applications/
update-desktop-database ~/.local/share/applications/
```

### Firewall Configuration (if needed)

Lilim runs on port 8080 (localhost only by default):

```bash
# If exposing to network (NOT RECOMMENDED without authentication)
sudo ufw allow 8080/tcp
```

---

## Troubleshooting Installation

### Build Failures

**Error**: "cargo: command not found"
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**Error**: "linker `cc` not found"
```bash
sudo apt install build-essential
```

**Error**: "openssl/ssl.h: No such file"
```bash
sudo apt install libssl-dev pkg-config
```

### Service Won't Start

Check logs:
```bash
sudo journalctl -u lilith-ai -n 50
```

Common issues:
- **Port 8080 already in use**: Change port in `/etc/lilith/lilim.yaml`
- **Permission denied**: Ensure `/var/lib/lilith` is owned by `lilith` user
- **Missing RAG indices**: Run `lilith_rag_builder` as shown above

### Frontend Build Issues

```bash
cd /home/blanco/LIL/LILIM/UI/chat-design

# Clear cache and rebuild
rm -rf node_modules package-lock.json
npm install
npm run build
```

---

## Uninstallation

```bash
sudo ./uninstall.sh
```

Options during uninstall:
- Preserve user data (`/var/lib/lilith`): Choose `y`
- Complete removal: Choose `n`

Manual removal (if script unavailable):
```bash
sudo systemctl stop lilith-ai
sudo systemctl disable lilith-ai
sudo rm /lib/systemd/system/lilith-ai.service
sudo rm /usr/bin/lilim*
sudo rm /usr/bin/lilith_*
sudo rm -rf /etc/lilith
sudo rm -rf /var/lib/lilith  # WARNING: Deletes all data
sudo userdel lilith
```

---

## Upgrading

1. Stop the service:
```bash
sudo systemctl stop lilith-ai
```

2. Backup configuration:
```bash
sudo cp /etc/lilith/lilim.yaml ~/lilim.yaml.backup
```

3. Pull/extract new version

4. Run installer (preserves config):
```bash
sudo ./install.sh
```

5. Restart service:
```bash
sudo systemctl start lilith-ai
```

---

## Directory Structure

```
/usr/bin/
├── lilim_server          # Main API server
├── lilith_rag_builder    # RAG index builder
├── lilith_search         # Search CLI tool
└── lilim                 # Main CLI interface

/etc/lilith/
└── lilim.yaml            # Configuration file

/var/lib/lilith/
├── rag_index/            # Vector embeddings
├── keyword_index/        # Full-text search
├── memory.db             # Conversation history
└── models/               # Future: local model weights

/usr/share/lilith/
├── lilim-responses.yaml  # Personality templates
└── web/                  # Frontend build (optional)

/lib/systemd/system/
└── lilith-ai.service     # Systemd unit file
```

---

## Next Steps

1. **Read the User Manual**: `/usr/share/doc/lilith-ai/USER_MANUAL.md`
2. **Test the system**: `lilim ask "What can you help me with?"`
3. **Configure API keys** (optional): Edit `/etc/lilith/lilim.yaml`
4. **Add custom RAG content**: Edit `rag_ready/rag_text.jsonl` and rebuild indices

---

**Installation complete! The flames of knowledge await you.**
