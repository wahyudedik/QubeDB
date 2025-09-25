#!/bin/bash
set -e

# Install directory
INSTALL_DIR="/opt/qubedb"
BIN_DIR="/usr/local/bin"
DESKTOP_DIR="$HOME/.local/share/applications"

# Create directories
sudo mkdir -p "$INSTALL_DIR"
sudo mkdir -p "$BIN_DIR"
mkdir -p "$DESKTOP_DIR"

# Copy binaries and icons
sudo cp qubedb-core "$INSTALL_DIR/"
sudo cp qubedb-gui "$INSTALL_DIR/"
sudo cp -r icons "$INSTALL_DIR/"

# Create symlinks for CLI
sudo ln -sf "$INSTALL_DIR/qubedb-core" "$BIN_DIR/qubedb-core"
sudo ln -sf "$INSTALL_DIR/qubedb-gui" "$BIN_DIR/qubedb-gui"

# Install desktop shortcut for GUI
cp qubedb.desktop "$DESKTOP_DIR/"

echo "QubeDB installed successfully!"
echo "Run 'qubedb-core' for CLI or search 'QubeDB Desktop' in your menu."
