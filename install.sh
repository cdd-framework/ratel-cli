#!/bin/bash

# Ratel CLI - Universal Installer
# Philosophy: Cyberattack-Driven Development (CDD)

set -e

# 1. Colors configuration
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🐾 Starting Ratel installation...${NC}"

# 2. OS detection
OS="linux"
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
    OS="windows"
fi

# 3. Fetching the latest version
# Note: Replace 'your-repo' with your actual GitHub repo
LATEST_RELEASE_URL="https://github.com/orgs/cdd-framework/ratel-cli/releases/latest/download/ratel-$OS-x64"

if [ "$OS" == "windows" ]; then
    LATEST_RELEASE_URL="${LATEST_RELEASE_URL}.exe"
fi

# 4. Downloading the binary
echo -e "Downloading Ratel for $OS..."
curl -L -o ratel "$LATEST_RELEASE_URL"

# 5. Permission and Moving (Global)
chmod +x ratel
if [ "$OS" != "windows" ]; then
    sudo mv ratel /usr/local/bin/ratel
    echo -e "${GREEN} Ratel has been installed in /usr/local/bin/ratel${NC}"
else
    echo -e "${GREEN} Ratel has been downloaded as ratel.exe${NC}"
fi

# 6. Conclusion
echo -e "\n${BLUE}Try it now:${NC}"
echo -e "  ratel --version"
echo -e "  ratel init"
echo -e "\n${GREEN}Welcome to the CDD movement!${NC}"