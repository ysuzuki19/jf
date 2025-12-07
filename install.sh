#!/bin/bash
set -e

REPO="ysuzuki19/jf"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  linux)
    case "$ARCH" in
      x86_64) ASSET="jf-linux-x86_64.tar.gz" ;;
      aarch64|arm64) ASSET="jf-linux-aarch64.tar.gz" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  darwin)
    case "$ARCH" in
      x86_64) ASSET="jf-darwin-x86_64.tar.gz" ;;
      arm64) ASSET="jf-darwin-aarch64.tar.gz" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

# Get latest release tag
LATEST_TAG=$(curl -sL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
  echo "Error: Could not fetch latest release"
  exit 1
fi

echo "Installing jf $LATEST_TAG..."

# Download and extract
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_TAG/$ASSET"
TMP_DIR=$(mktemp -d)

echo "Downloading from $DOWNLOAD_URL"
curl -sL "$DOWNLOAD_URL" -o "$TMP_DIR/$ASSET"

echo "Extracting..."
tar xzf "$TMP_DIR/$ASSET" -C "$TMP_DIR"

# Install
mkdir -p "$INSTALL_DIR"
mv "$TMP_DIR/jf" "$INSTALL_DIR/jf"
chmod +x "$INSTALL_DIR/jf"

# Cleanup
rm -rf "$TMP_DIR"

echo "Successfully installed jf to $INSTALL_DIR/jf"

# Check if INSTALL_DIR is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  echo ""
  echo "Note: $INSTALL_DIR is not in your PATH."
  echo "Add it to your shell config:"
  echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
fi
