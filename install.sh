#!/bin/bash

# Ratel CLI - Universal Installer (User-friendly version)
# Philosophy: Cyberattack-Driven Development (CDD)

set -e

# 1. Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🐾 Starting Ratel installation...${NC}"

# 2. DDetection of the OS
OS="linux"
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
    OS="windows"
fi

# 3. Construction of the download URL (Corrected URL)
REPO_URL="https://github.com/cdd-framework/ratel-cli/releases/latest/download"

if [ "$OS" == "windows" ]; then
    ASSET_NAME="ratel-windows-x64.exe"
    BINARY_NAME="ratel.exe"
elif [ "$OS" == "macos" ]; then
    ASSET_NAME="ratel-macos-x64"
    BINARY_NAME="ratel"
else
    ASSET_NAME="ratel-linux-x64"
    BINARY_NAME="ratel"
fi

DOWNLOAD_URL="${REPO_URL}/${ASSET_NAME}"

# 4. Download the binary
echo -e "Downloading Ratel from: ${DOWNLOAD_URL}"
curl -L -s -o ratel_tmp "$DOWNLOAD_URL"

# VVerification: if the file contains "Not Found", the URL is incorrect
if grep -q "Not Found" ratel_tmp; then
    echo -e "${RED} Error: Binary not found at URL. Check your GitHub release tags.${NC}"
    rm ratel_tmp
    exit 1
fi

chmod +x ratel_tmp

# 5. Local installation (Without Administrator rights)
# We use the 'bin' folder in the user's home directory
INSTALL_DIR="$HOME/bin"
mkdir -p "$INSTALL_DIR"

mv ratel_tmp "$INSTALL_DIR/$BINARY_NAME"

# 6. Update the PATH for the terminal (Bash / Git Bash)
# We check if $HOME/bin is already in the PATH
if [[ ":$PATH:" != *":$HOME/bin:"* ]]; then
    # DDetect the appropriate configuration file
    if [ -f "$HOME/.bashrc" ]; then
        CONF_FILE="$HOME/.bashrc"
    elif [ -f "$HOME/.bash_profile" ]; then
        CONF_FILE="$HOME/.bash_profile"
    else
        CONF_FILE="$HOME/.profile"
    fi
    
    echo "export PATH=\"\$HOME/bin:\$PATH\"" >> "$CONF_FILE"
    echo -e "${BLUE} PATH updated in $CONF_FILE${NC}"
fi

# 7. Conclusion
echo -e "\n${GREEN}Ratel has been installed in $INSTALL_DIR/$BINARY_NAME${NC}"
echo -e "${BLUE}To activate the command immediately, type: ${NC} source $CONF_FILE"
echo -e "\nTry it now:"
echo -e "  ratel --version"
echo -e "  ratel init"
echo -e "\n${GREEN}Welcome to the CDD movement!${NC}"