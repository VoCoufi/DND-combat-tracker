#!/bin/sh
# Installation script for dnd-combat-tracker
# Usage: curl -sSL https://github.com/VoCoufi/DND-combat-tracker/raw/master/install.sh | sh

set -e

# Configuration
REPO="VoCoufi/DND-combat-tracker"
BINARY_NAME="dnd-combat-tracker"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Colors (disabled if not a terminal)
if [ -t 1 ]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[0;33m'
    BLUE='\033[0;34m'
    NC='\033[0m' # No Color
else
    RED=''
    GREEN=''
    YELLOW=''
    BLUE=''
    NC=''
fi

info() {
    printf "${BLUE}info:${NC} %s\n" "$1"
}

success() {
    printf "${GREEN}success:${NC} %s\n" "$1"
}

warn() {
    printf "${YELLOW}warning:${NC} %s\n" "$1"
}

error() {
    printf "${RED}error:${NC} %s\n" "$1" >&2
    exit 1
}

usage() {
    cat <<EOF
Install script for dnd-combat-tracker

Usage:
    install.sh [OPTIONS]

Options:
    -h, --help      Show this help message
    -v, --version   Install a specific version (e.g., v0.5.0)
    -d, --dir       Installation directory (default: ~/.local/bin)
    --uninstall     Remove dnd-combat-tracker

Examples:
    # Install latest version
    curl -sSL https://github.com/VoCoufi/DND-combat-tracker/raw/master/install.sh | sh

    # Install specific version
    curl -sSL https://github.com/VoCoufi/DND-combat-tracker/raw/master/install.sh | sh -s -- -v v0.5.0

    # Install to custom directory
    curl -sSL https://github.com/VoCoufi/DND-combat-tracker/raw/master/install.sh | sh -s -- -d /usr/local/bin

    # Uninstall
    curl -sSL https://github.com/VoCoufi/DND-combat-tracker/raw/master/install.sh | sh -s -- --uninstall
EOF
    exit 0
}

# Parse arguments
VERSION=""
UNINSTALL=false
while [ $# -gt 0 ]; do
    case "$1" in
        -h|--help)
            usage
            ;;
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        -d|--dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        --uninstall)
            UNINSTALL=true
            shift
            ;;
        *)
            error "Unknown option: $1"
            ;;
    esac
done

# Uninstall function
uninstall() {
    info "Uninstalling $BINARY_NAME..."

    BINARY_PATH="$INSTALL_DIR/$BINARY_NAME"

    if [ ! -f "$BINARY_PATH" ]; then
        error "$BINARY_NAME not found at $BINARY_PATH"
    fi

    rm -f "$BINARY_PATH"
    success "Successfully removed $BINARY_PATH"
}

# Run uninstall if requested
if [ "$UNINSTALL" = true ]; then
    uninstall
    exit 0
fi

# Detect OS
detect_os() {
    OS="$(uname -s)"
    case "$OS" in
        Linux*)  OS="linux" ;;
        Darwin*) OS="macos" ;;
        *)       error "Unsupported operating system: $OS" ;;
    esac
    echo "$OS"
}

# Detect architecture
detect_arch() {
    ARCH="$(uname -m)"
    case "$ARCH" in
        x86_64|amd64)     ARCH="x86_64" ;;
        arm64|aarch64)    ARCH="aarch64" ;;
        *)                error "Unsupported architecture: $ARCH" ;;
    esac
    echo "$ARCH"
}

# Check for required commands
check_requirements() {
    if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
        error "Either curl or wget is required but not found"
    fi

    if ! command -v tar >/dev/null 2>&1; then
        error "tar is required but not found"
    fi

    if ! command -v sha256sum >/dev/null 2>&1 && ! command -v shasum >/dev/null 2>&1; then
        error "sha256sum or shasum is required but not found"
    fi
}

# Download file using curl or wget
download() {
    url="$1"
    output="$2"

    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$url" -o "$output"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "$url" -O "$output"
    fi
}

# Calculate SHA256 checksum
sha256() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | cut -d' ' -f1
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 "$1" | cut -d' ' -f1
    fi
}

# Get latest version from GitHub API
get_latest_version() {
    if command -v curl >/dev/null 2>&1; then
        curl -sS "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | cut -d'"' -f4
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | cut -d'"' -f4
    fi
}

main() {
    info "Installing $BINARY_NAME..."

    check_requirements

    OS=$(detect_os)
    ARCH=$(detect_arch)

    # Determine asset name
    if [ "$OS" = "macos" ]; then
        ASSET="dnd-combat-tracker-macos-${ARCH}.tar.gz"
    else
        ASSET="dnd-combat-tracker-${OS}-${ARCH}.tar.gz"
    fi

    info "Detected platform: $OS-$ARCH"

    # Get version
    if [ -z "$VERSION" ]; then
        info "Fetching latest version..."
        VERSION=$(get_latest_version)
        if [ -z "$VERSION" ]; then
            error "Failed to fetch latest version"
        fi
    fi

    info "Installing version: $VERSION"

    # Create temp directory
    TMP_DIR=$(mktemp -d)
    trap 'rm -rf "$TMP_DIR"' EXIT

    # Download URLs
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$ASSET"
    CHECKSUM_URL="https://github.com/$REPO/releases/download/$VERSION/checksums.txt"

    # Download archive
    info "Downloading $ASSET..."
    if ! download "$DOWNLOAD_URL" "$TMP_DIR/$ASSET"; then
        error "Failed to download $DOWNLOAD_URL"
    fi

    # Download and verify checksum
    info "Verifying checksum..."
    if download "$CHECKSUM_URL" "$TMP_DIR/checksums.txt" 2>/dev/null; then
        EXPECTED=$(grep "$ASSET" "$TMP_DIR/checksums.txt" | cut -d' ' -f1)
        ACTUAL=$(sha256 "$TMP_DIR/$ASSET")

        if [ "$EXPECTED" != "$ACTUAL" ]; then
            error "Checksum verification failed!\nExpected: $EXPECTED\nActual:   $ACTUAL"
        fi
        success "Checksum verified"
    else
        warn "Checksums not available for this release, skipping verification"
    fi

    # Extract archive
    info "Extracting archive..."
    tar -xzf "$TMP_DIR/$ASSET" -C "$TMP_DIR"

    # Create install directory if needed
    if [ ! -d "$INSTALL_DIR" ]; then
        info "Creating directory: $INSTALL_DIR"
        mkdir -p "$INSTALL_DIR"
    fi

    # Install binary
    info "Installing to $INSTALL_DIR/$BINARY_NAME"
    mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"

    success "Successfully installed $BINARY_NAME $VERSION"

    # Check if install directory is in PATH
    case ":$PATH:" in
        *":$INSTALL_DIR:"*)
            info "Run '$BINARY_NAME' to start"
            ;;
        *)
            echo ""
            warn "$INSTALL_DIR is not in your PATH"
            echo "Add it to your shell configuration:"
            echo ""
            echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
            echo ""
            ;;
    esac
}

main "$@"
