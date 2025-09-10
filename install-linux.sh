#!/usr/bin/env bash

set -e

if [[ "$(uname -s)" != "Linux" ]]; then
  echo "This installer only supports Linux."
  exit 1
fi

ARCH=$(uname -m)
case "$ARCH" in
  x86_64) PLATFORM="x86_64" ;;
  aarch64 | arm64) PLATFORM="arm64" ;;
  *)
    echo "Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

URL="https://github.com/kevinj045/rew/releases/latest/download/rew-linux_${PLATFORM}.tar.gz"

TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

echo "Downloading rew for $PLATFORM..."
curl -L -o rew.tar.gz "$URL"

echo "Extracting..."
tar -xzf rew.tar.gz

INSTALL_DIR="/usr/local/bin"
if [[ $EUID -ne 0 ]]; then
  echo "Installing to $INSTALL_DIR requires root privileges. Asking for sudo..."
  sudo mv rew "$INSTALL_DIR/rew"
else
  mv rew "$INSTALL_DIR/rew"
fi

chmod +x "$INSTALL_DIR/rew"

echo "✅ rew installed to $INSTALL_DIR/rew"
rew --version || echo "Try restarting your shell if 'rew' is not yet in PATH."
