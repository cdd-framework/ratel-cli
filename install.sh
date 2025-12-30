#!/bin/bash

# Ratel CLI - Universal Installer
# Philosophy: Cyberattack-Driven Development (CDD)

set -e

# 1. Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🐾 Starting Ratel installation...${NC}"

# 2. Détection de l'OS
OS="linux"
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
    OS="windows"
fi

# 3. Download URL Construction
REPO_URL="https://github.com/cdd-framework/ratel-cli/releases/latest/download"

if [ "$OS" == "windows" ]; then
    ASSET_NAME="ratel-windows-x64.exe"
elif [ "$OS" == "macos" ]; then
    ASSET_NAME="ratel-macos-x64"
else
    ASSET_NAME="ratel-linux-x64"
fi

DOWNLOAD_URL="${REPO_URL}/${ASSET_NAME}"

# 4. Downloading the binary
echo -e "Downloading Ratel from: ${DOWNLOAD_URL}"
# Using a temporary file for verification
curl -L -s -o ratel_tmp "$DOWNLOAD_URL"

# Verification: if the file contains "Not Found", the URL is still incorrect
if grep -q "Not Found" ratel_tmp; then
    echo -e "${RED}Error: Binary not found at URL. Please check your GitHub release tags.${NC}"
    rm ratel_tmp
    exit 1
fi

# 5. Permission and Global Installation
chmod +x ratel_tmp

if [ "$OS" == "windows" ]; then
    # Moving to /usr/bin for immediate global access in Git Bash
    mv ratel_tmp /usr/bin/ratel.exe
    echo -e "${GREEN}Ratel has been installed globally in Git Bash (/usr/bin/ratel.exe)${NC}"
else
    sudo mv ratel_tmp /usr/local/bin/ratel
    echo -e "${GREEN}Ratel has been installed in /usr/local/bin/ratel${NC}"
fi

# 6. Conclusion
echo -e "\n${BLUE}Installation complete! Try it now:${NC}"
echo -e "  ratel --version"
echo -e "  ratel init"
echo -e "\n${GREEN}Welcome to the CDD movement!${NC}"