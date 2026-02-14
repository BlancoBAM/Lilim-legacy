#!/usr/bin/env bash
#
# Lilim AI Assistant - Uninstallation Script
# Removes Lilim from the system
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${RED}╔═══════════════════════════════════════╗${NC}"
echo -e "${RED}║   Lilim AI Assistant Uninstaller     ║${NC}"
echo -e "${RED}╚═══════════════════════════════════════╝${NC}"
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
   echo -e "${RED}[-] Please run as root (sudo ./uninstall.sh)${NC}"
   exit 1
fi

# Ask about data preservation
echo -e "${YELLOW}[?] Do you want to preserve user data (/var/lib/lilith)? [y/N]${NC}"
read -r preserve_data

# Stop and disable service
echo -e "${YELLOW}[*] Stopping Lilim service...${NC}"
if systemctl is-active --quiet lilith-ai; then
    systemctl stop lilith-ai
    echo -e "${GREEN}[+] Service stopped${NC}"
fi

if systemctl is-enabled --quiet lilith-ai 2>/dev/null; then
    systemctl disable lilith-ai
    echo -e "${GREEN}[+] Service disabled${NC}"
fi

# Remove systemd service
if [ -f /lib/systemd/system/lilith-ai.service ]; then
    rm /lib/systemd/system/lilith-ai.service
    systemctl daemon-reload
    echo -e "${GREEN}[+] Systemd service removed${NC}"
fi

# Remove binaries
echo -e "${YELLOW}[*] Removing binaries...${NC}"
rm -f /usr/bin/lilim_server
rm -f /usr/bin/lilith_rag_builder
rm -f /usr/bin/lilith_search
rm -f /usr/bin/lilim
echo -e "${GREEN}[+] Binaries removed${NC}"

# Remove desktop integration
echo -e "${YELLOW}[*] Removing desktop integration...${NC}"
rm -f /usr/share/applications/lilith-ai.desktop
rm -rf /usr/share/icons/hicolor/*/apps/lilith-ai.*
echo -e "${GREEN}[+] Desktop files removed${NC}"

# Remove documentation
rm -rf /usr/share/doc/lilith-ai
rm -rf /usr/share/lilith
echo -e "${GREEN}[+] Documentation removed${NC}"

# Remove configuration
if [ -f /etc/lilith/lilim.yaml ]; then
    echo -e "${YELLOW}[*] Removing configuration...${NC}"
    rm /etc/lilith/lilim.yaml
    rmdir /etc/lilith 2>/dev/null || true
    echo -e "${GREEN}[+] Configuration removed${NC}"
fi

# Handle user data
if [[ ! "$preserve_data" =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}[*] Removing user data...${NC}"
    rm -rf /var/lib/lilith
    echo -e "${GREEN}[+] User data removed${NC}"
else
    echo -e "${YELLOW}[!] User data preserved in /var/lib/lilith${NC}"
fi

# Remove logs
rm -rf /var/log/lilith
echo -e "${GREEN}[+] Logs removed${NC}"

# Remove system user (if no data preserved)
if [[ ! "$preserve_data" =~ ^[Yy]$ ]]; then
    if id -u lilith > /dev/null 2>&1; then
        userdel lilith
        echo -e "${GREEN}[+] System user 'lilith' removed${NC}"
    fi
fi

echo ""
echo -e "${GREEN}╔═══════════════════════════════════════╗${NC}"
echo -e "${GREEN}║   Uninstallation Complete!            ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════╝${NC}"
echo ""

if [[ "$preserve_data" =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}User data preserved in:${NC}"
    echo -e "  /var/lib/lilith/"
    echo -e ""
    echo -e "To completely remove all traces:"
    echo -e "  ${RED}sudo rm -rf /var/lib/lilith${NC}"
    echo -e "  ${RED}sudo userdel lilith${NC}"
    echo -e ""
fi
