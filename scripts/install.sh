#!/bin/sh
# slice installer script
# Usage: curl -fsSL https://raw.githubusercontent.com/rosshhun/slice/main/scripts/install.sh | sh

set -e

# 1. Detect OS and Arch
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)  PLATFORM="unknown-linux-gnu" ;;
    Darwin) PLATFORM="apple-darwin" ;;
    *)      echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
    x86_64)  ARCH="x86_64" ;;
    arm64|aarch64) ARCH="aarch64" ;;
    *)       echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

BINARY="slice"
REPO="rosshhun/slice"

# Fetch the latest tag from GitHub API
echo "🔎 Checking for latest release..."
TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$TAG" ]; then
    echo "Error: Could not find latest release tag."
    exit 1
fi

URL="https://github.com/$REPO/releases/download/$TAG/slice-$TAG-$ARCH-$PLATFORM.tar.gz"

echo "⬇️  Downloading $BINARY $TAG for $OS ($ARCH)..."

# 2. Download and Unzip (to a temp folder)
TMP_DIR=$(mktemp -d)
if ! curl -sL "$URL" | tar xz -C "$TMP_DIR"; then
    echo "❌ Download failed. Check your network or platform support."
    exit 1
fi

# 3. Install
INSTALL_DIR="/usr/local/bin"

echo "📦 Installing to $INSTALL_DIR..."
if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP_DIR/$BINARY" "$INSTALL_DIR/$BINARY"
else
    # Sudo is required if we don't own /usr/local/bin
    sudo mv "$TMP_DIR/$BINARY" "$INSTALL_DIR/$BINARY"
fi

# 4. Cleanup and Verify
rm -rf "$TMP_DIR"
echo "✅ Successfully installed $BINARY $TAG!"
$BINARY --version