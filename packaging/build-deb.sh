#!/bin/bash
# Build script for creating lilim-ai .deb package

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
PKG_DIR="$SCRIPT_DIR/debian"
VERSION="0.1.0"

echo "Building Lilim AI Assistant v$VERSION..."

# Clean previous build
rm -rf "$PKG_DIR/usr" "$PKG_DIR/etc"
mkdir -p "$PKG_DIR/usr/bin"
mkdir -p "$PKG_DIR/usr/lib/lilim"
mkdir -p "$PKG_DIR/usr/share/applications"
mkdir -p "$PKG_DIR/usr/share/icons/hicolor/256x256/apps"
mkdir -p "$PKG_DIR/usr/share/doc/lilim-ai"
mkdir -p "$PKG_DIR/etc/lilith"
mkdir -p "$PKG_DIR/lib/systemd/system"

# Build Rust workspace
echo "Building Rust components..."
cd "$PROJECT_ROOT"
cargo build --release --workspace || {
    echo "Error: Failed to build Rust workspace"
    exit 1
}

# Build Tauri desktop app (if dependencies are installed)
echo "Building Tauri desktop app..."
cd "$PROJECT_ROOT/lilim_desktop"
if npm run tauri build 2>/dev/null; then
    echo "Tauri build successful"
    # Copy Tauri binary
    if [ -f "src-tauri/target/release/lilim-desktop" ]; then
        cp "src-tauri/target/release/lilim-desktop" "$PKG_DIR/usr/bin/"
    fi
else
    echo "Warning: Tauri build failed (missing GTK dependencies?)"
    echo "Desktop app will not be included in package"
fi

# Copy binaries
cp "$PROJECT_ROOT/target/release/lilim_server" "$PKG_DIR/usr/lib/lilim/" || true
cp "$PROJECT_ROOT/target/release/lilith_rag_builder" "$PKG_DIR/usr/lib/lilim/" || true
cp "$PROJECT_ROOT/target/release/lilith_search" "$PKG_DIR/usr/lib/lilim/" || true

# Copy configuration
cp "$PROJECT_ROOT/config/lilim.yaml" "$PKG_DIR/etc/lilith/"

# Copy desktop file
cp "$PROJECT_ROOT/desktop/lilith-ai.desktop" "$PKG_DIR/usr/share/applications/lilim.desktop"

# Copy systemd service
cp "$PROJECT_ROOT/systemd/system/lilith-ai.service" "$PKG_DIR/lib/systemd/system/"

# Copy documentation
cp "$PROJECT_ROOT/docs/USER_MANUAL.md" "$PKG_DIR/usr/share/doc/lilim-ai/README.md"
cp "$PROJECT_ROOT/docs/INSTALL_GUIDE.md" "$PKG_DIR/usr/share/doc/lilim-ai/"
cp "$PROJECT_ROOT/docs/API_REFERENCE.md" "$PKG_DIR/usr/share/doc/lilim-ai/"

# Create icon (placeholder - should be replaced with actual icon)
echo "Note: Using placeholder icon. Replace with actual Lilim icon."
# Convert or copy icon file
# cp "$PROJECT_ROOT/desktop/icons/lilith-256.png" "$PKG_DIR/usr/share/icons/hicolor/256x256/apps/lilith-ai.png"

# Set permissions
find "$PKG_DIR/usr/bin" -type f -exec chmod 755 {} \;
find "$PKG_DIR/usr/lib/lilim" -type f -exec chmod 755 {} \;
chmod 644 "$PKG_DIR/etc/lilith/lilim.yaml"
chmod 644 "$PKG_DIR/lib/systemd/system/lilith-ai.service"

# Build .deb package
echo "Creating .deb package..."
cd "$SCRIPT_DIR"
dpkg-deb --build debian "lilim-ai_${VERSION}_amd64.deb"

echo ""
echo "╔════════════════════════════════════════════════════════╗"
echo "║  Package created: lilim-ai_${VERSION}_amd64.deb"
echo "╚════════════════════════════════════════════════════════╝"
echo ""
echo "Install with:"
echo "  sudo dpkg -i lilim-ai_${VERSION}_amd64.deb"
echo "  sudo apt-get install -f  # Fix any missing dependencies"
echo ""
