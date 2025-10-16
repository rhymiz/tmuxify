#!/usr/bin/env bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Installation directory
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
BINARY_PATH="$INSTALL_DIR/tmuxify"

echo -e "${CYAN}tmuxify uninstaller${NC}"
echo ""

if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${YELLOW}tmuxify not found at $BINARY_PATH${NC}"
    echo ""
    echo "If you installed to a different location, set INSTALL_DIR:"
    echo -e "  ${CYAN}INSTALL_DIR=/usr/local/bin ./uninstall.sh${NC}"
    exit 1
fi

echo -e "Found tmuxify at: ${CYAN}$BINARY_PATH${NC}"
echo ""
read -p "Remove tmuxify? [y/N] " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    rm "$BINARY_PATH"
    echo -e "${GREEN}✓${NC} tmuxify removed successfully"
else
    echo "Uninstall cancelled"
    exit 0
fi
