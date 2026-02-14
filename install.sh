#!/usr/bin/env bash
#
# Lilim AI Assistant - Installation Script
# Installs Lilim system-wide on Lilith Linux
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}╔═══════════════════════════════════════╗${NC}"
echo -e "${GREEN}║   Lilim AI Assistant Installer       ║${NC}"
echo -e "${GREEN}║   Lilith Linux System Integration    ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════╝${NC}"
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
   echo -e "${RED}[-] Please run as root (sudo ./install.sh)${NC}"
   exit 1
fi

# Get the script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${YELLOW}[*] Building Lilim components...${NC}"

# Build all Rust binaries in release mode
cd "$SCRIPT_DIR"
cargo build --release --workspace

if [ $? -ne 0 ]; then
    echo -e "${RED}[-] Build failed!${NC}"
    exit 1
fi

echo -e "${GREEN}[+] Build successful${NC}"

# Create system user and group
echo -e "${YELLOW}[*] Creating lilith system user...${NC}"
if ! id -u lilith > /dev/null 2>&1; then
    useradd --system --no-create-home --shell /usr/sbin/nologin lilith
    echo -e "${GREEN}[+] User 'lilith' created${NC}"
else
    echo -e "${GREEN}[+] User 'lilith' already exists${NC}"
fi

# Create directory structure
echo -e "${YELLOW}[*] Creating directory structure...${NC}"
mkdir -p /usr/bin
mkdir -p /etc/lilith
mkdir -p /var/lib/lilith/{rag_index,keyword_index,models}
mkdir -p /var/log/lilith
mkdir -p /usr/share/lilith/rag_source
mkdir -p /usr/share/doc/lilith-ai
mkdir -p /usr/share/applications
mkdir -p /usr/share/icons/hicolor/{16x16,32x32,48x48,64x64,128x128,256x256}/apps

# Install binaries
echo -e "${YELLOW}[*] Installing binaries...${NC}"
install -m 755 target/release/lilim_server /usr/bin/lilim_server
install -m 755 target/release/lilith_rag_builder /usr/bin/lilith_rag_builder
install -m 755 target/release/lilith_search /usr/bin/lilith_search

# Install CLI if built
if [ -f "target/release/lilim" ]; then
    install -m 755 target/release/lilim /usr/bin/lilim
    echo -e "${GREEN}[+] CLI tool installed${NC}"
fi

echo -e "${GREEN}[+] Binaries installed to /usr/bin/${NC}"

# Install configuration template
echo -e "${YELLOW}[*] Installing configuration...${NC}"
if [ ! -f /etc/lilith/lilim.yaml ]; then
    if [ -f "$SCRIPT_DIR/config/lilim.yaml" ]; then
        install -m 644 "$SCRIPT_DIR/config/lilim.yaml" /etc/lilith/lilim.yaml
        echo -e "${GREEN}[+] Configuration installed to /etc/lilith/lilim.yaml${NC}"
    else
        cat > /etc/lilith/lilim.yaml << 'EOF'
# Lilim AI Assistant Configuration

models:
  primary: "api-fallback"
  fallback_provider: "openai"
  crane_model_path: "/var/lib/lilith/models/crane"

rag:
  vector_index: "/var/lib/lilith/rag_index"
  keyword_index: "/var/lib/lilith/keyword_index"
  max_results: 5

memory:
  db_path: "/var/lib/lilith/memory.db"
  max_history_messages: 20

agent:
  enable_web_search: true
  enable_file_search: true
  enable_terminal: true
  yolo_mode: false

api_keys:
  openai: null
  anthropic: null

server:
  host: "127.0.0.1"
  port: 8080

personality:
  responses_yaml_path: "/usr/share/lilith/lilim-responses.yaml"

routing:
  complexity_threshold: 0.7
EOF
        chmod 644 /etc/lilith/lilim.yaml
        echo -e "${GREEN}[+] Default configuration created${NC}"
    fi
else
    echo -e "${YELLOW}[!] Configuration already exists, skipping${NC}"
fi

# Install personality responses
if [ -f "$SCRIPT_DIR/lilim-responses.yaml" ]; then
    install -m 644 "$SCRIPT_DIR/lilim-responses.yaml" /usr/share/lilith/
    echo -e "${GREEN}[+] Personality responses installed${NC}"
fi

# Install RAG data if exists
if [ -d "$SCRIPT_DIR/lilith_rag_builder/rag_ready" ]; then
    echo -e "${YELLOW}[*] Copying RAG data...${NC}"
    cp -r "$SCRIPT_DIR/lilith_rag_builder/rag_ready"/* /var/lib/lilith/ 2>/dev/null || true
    echo -e "${GREEN}[+] RAG indices copied${NC}"
fi

# Install systemd service
echo -e "${YELLOW}[*] Installing systemd service...${NC}"
install -m 644 "$SCRIPT_DIR/systemd/system/lilith-ai.service" /lib/systemd/system/lilith-ai.service
systemctl daemon-reload
echo -e "${GREEN}[+] Systemd service installed${NC}"

# Install desktop file if exists
if [ -f "$SCRIPT_DIR/desktop/lilith-ai.desktop" ]; then
    install -m 644 "$SCRIPT_DIR/desktop/lilith-ai.desktop" /usr/share/applications/
    echo -e "${GREEN}[+] Desktop launcher installed${NC}"
fi

# Install documentation
if [ -d "$SCRIPT_DIR/docs" ]; then
    cp -r "$SCRIPT_DIR/docs"/* /usr/share/doc/lilith-ai/ 2>/dev/null || true
    echo -e "${GREEN}[+] Documentation installed${NC}"
fi

# Set ownership
echo -e "${YELLOW}[*] Setting permissions...${NC}"
chown -R lilith:lilith /var/lib/lilith
chown -R lilith:lilith /var/log/lilith
chmod 755 /var/lib/lilith
chmod 750 /var/lib/lilith/{rag_index,keyword_index,models}
chmod 640 /etc/lilith/lilim.yaml

echo -e "${GREEN}[+] Permissions configured${NC}"

# Initialize database if needed
if [ ! -f /var/lib/lilith/memory.db ]; then
    echo -e "${YELLOW}[*] Initializing memory database...${NC}"
    touch /var/lib/lilith/memory.db
    chown lilith:lilith /var/lib/lilith/memory.db
    chmod 640 /var/lib/lilith/memory.db
    echo -e "${GREEN}[+] Database initialized${NC}"
fi

echo ""
echo -e "${GREEN}╔═══════════════════════════════════════╗${NC}"
echo -e "${GREEN}║   Installation Complete!              ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════╝${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo -e "  1. Edit /etc/lilith/lilim.yaml to configure API keys (optional)"
echo -e "  2. Start the service: ${GREEN}sudo systemctl start lilith-ai${NC}"
echo -e "  3. Enable auto-start: ${GREEN}sudo systemctl enable lilith-ai${NC}"
echo -e "  4. Check status: ${GREEN}sudo systemctl status lilith-ai${NC}"
echo -e "  5. View logs: ${GREEN}sudo journalctl -u lilith-ai -f${NC}"
echo -e "  6. Access UI: ${GREEN}http://localhost:8080${NC}"
echo ""
if [ -f /usr/bin/lilim ]; then
    echo -e "CLI tool installed: ${GREEN}lilim ask \"your question\"${NC}"
    echo ""
fi
