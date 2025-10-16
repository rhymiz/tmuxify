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

echo -e "${CYAN}tmuxify installer${NC}"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo not found${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

echo -e "${GREEN}✓${NC} Found cargo"

# Build the project
echo ""
echo -e "${CYAN}Building tmuxify...${NC}"
cargo build --release

if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Build failed${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} Build successful"

# Create installation directory if it doesn't exist
if [ ! -d "$INSTALL_DIR" ]; then
    echo ""
    echo -e "${YELLOW}Creating installation directory: $INSTALL_DIR${NC}"
    mkdir -p "$INSTALL_DIR"
fi

# Copy binary
echo ""
echo -e "${CYAN}Installing tmuxify to $INSTALL_DIR${NC}"
cp target/release/tmuxify "$INSTALL_DIR/tmuxify"

if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Failed to copy binary${NC}"
    exit 1
fi

chmod +x "$INSTALL_DIR/tmuxify"

echo -e "${GREEN}✓${NC} Installation complete"

# Check if install directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo -e "${YELLOW}Warning: $INSTALL_DIR is not in your PATH${NC}"
    echo ""
    echo "Add this line to your shell configuration file (~/.zshrc or ~/.bashrc):"
    echo -e "${CYAN}export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
else
    echo -e "${GREEN}✓${NC} $INSTALL_DIR is in your PATH"
fi

# Check if tmuxify can be executed
echo ""
if command -v tmuxify &> /dev/null; then
    echo -e "${GREEN}✓${NC} tmuxify is ready to use!"
    echo ""
    echo "Run these commands to get started:"
    echo -e "  ${CYAN}tmuxify doctor${NC}    - Check system dependencies"
    echo -e "  ${CYAN}tmuxify --help${NC}    - Show usage information"
    echo -e "  ${CYAN}tmuxify${NC}           - Start interactive wizard"
else
    echo -e "${YELLOW}Installation complete. You may need to restart your shell.${NC}"
    echo ""
    echo "After restarting, run:"
    echo -e "  ${CYAN}tmuxify doctor${NC}    - Check system dependencies"
fi

echo ""
