#!/bin/bash

set -e

# Configuration
REPO="aurabx/runbeam-cli"
BINARY_NAME="runbeam"
GITHUB_API="https://api.github.com/repos"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

log_success() {
    echo -e "${GREEN}✓${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

log_error() {
    echo -e "${RED}✗${NC} $1"
}

# Detect OS and architecture
detect_platform() {
    local os=$(uname -s)
    local arch=$(uname -m)
    
    case "$os" in
        Darwin)
            OS="macos"
            case "$arch" in
                arm64|aarch64)
                    ARCH="aarch64"
                    ;;
                x86_64)
                    ARCH="x86_64"
                    ;;
                *)
                    log_error "Unsupported macOS architecture: $arch"
                    exit 1
                    ;;
            esac
            ;;
        Linux)
            OS="linux"
            case "$arch" in
                x86_64)
                    ARCH="x86_64"
                    ;;
                aarch64|arm64)
                    ARCH="aarch64"
                    log_warning "aarch64 Linux builds not available. Please build from source."
                    exit 1
                    ;;
                *)
                    log_error "Unsupported Linux architecture: $arch"
                    exit 1
                    ;;
            esac
            ;;
        MINGW64_NT*|MSYS_NT*)
            OS="windows"
            ARCH="x86_64"
            ;;
        *)
            log_error "Unsupported OS: $os"
            exit 1
            ;;
    esac
    
    log_info "Detected platform: $OS ($ARCH)"
}

# Get the latest release version
get_latest_release() {
    log_info "Fetching latest release information..."
    
    local response=$(curl -s "${GITHUB_API}/${REPO}/releases/latest")
    
    if echo "$response" | grep -q "Not Found"; then
        log_error "Repository not found or has no releases"
        exit 1
    fi
    
    VERSION=$(echo "$response" | grep '"tag_name"' | head -1 | cut -d'"' -f4)
    
    if [ -z "$VERSION" ]; then
        log_error "Failed to determine latest version"
        exit 1
    fi
    
    log_info "Latest version: $VERSION"
}

# Construct download URL
get_download_url() {
    case "$OS" in
        macos)
            FILENAME="${BINARY_NAME}-macos-${ARCH}-${VERSION}.tar.gz"
            URL="${GITHUB_API}/${REPO}/releases/download/${VERSION}/${FILENAME}"
            CHECKSUM_FILE="${FILENAME}.sha256"
            ;;
        linux)
            FILENAME="${BINARY_NAME}-linux-${ARCH}-${VERSION}.tar.gz"
            URL="${GITHUB_API}/${REPO}/releases/download/${VERSION}/${FILENAME}"
            CHECKSUM_FILE="${FILENAME}.sha256"
            ;;
        windows)
            FILENAME="${BINARY_NAME}-windows-${ARCH}-${VERSION}.zip"
            URL="${GITHUB_API}/${REPO}/releases/download/${VERSION}/${FILENAME}"
            CHECKSUM_FILE="${FILENAME}.sha256"
            ;;
    esac
    
    # Use releases URL directly instead of API for actual downloads
    URL="https://github.com/${REPO}/releases/download/${VERSION}/${FILENAME}"
    CHECKSUM_URL="https://github.com/${REPO}/releases/download/${VERSION}/${CHECKSUM_FILE}"
}

# Download file with fallback
download_file() {
    local url=$1
    local output=$2
    
    log_info "Downloading from: $url"
    
    if ! curl -fsSL "$url" -o "$output" 2>/dev/null; then
        # Try with wget as fallback
        if command -v wget &> /dev/null; then
            log_info "curl failed, trying wget..."
            if ! wget -q "$url" -O "$output"; then
                log_error "Failed to download from $url"
                rm -f "$output"
                return 1
            fi
        else
            log_error "Failed to download from $url (curl and wget not available)"
            return 1
        fi
    fi
    
    return 0
}

# Verify checksum
verify_checksum() {
    local file=$1
    local checksum_file=$2
    
    log_info "Verifying checksum..."
    
    # Download checksum file
    if ! download_file "$CHECKSUM_URL" "$TMP_DIR/$CHECKSUM_FILE"; then
        log_warning "Could not download checksum file, skipping verification"
        return 0
    fi
    
    # Verify
    if command -v sha256sum &> /dev/null; then
        cd "$TMP_DIR"
        if ! sha256sum -c "$CHECKSUM_FILE" > /dev/null 2>&1; then
            log_error "Checksum verification failed"
            return 1
        fi
    elif command -v shasum &> /dev/null; then
        cd "$TMP_DIR"
        if ! shasum -a 256 -c "$CHECKSUM_FILE" > /dev/null 2>&1; then
            log_error "Checksum verification failed"
            return 1
        fi
    else
        log_warning "sha256sum/shasum not found, skipping checksum verification"
    fi
    
    log_success "Checksum verified"
    return 0
}

# Extract archive
extract_archive() {
    local file=$1
    local dest=$2
    
    log_info "Extracting archive..."
    
    case "$OS" in
        windows)
            if command -v unzip &> /dev/null; then
                unzip -q "$file" -d "$dest"
            else
                log_error "unzip command not found"
                return 1
            fi
            ;;
        *)
            if ! tar -xzf "$file" -C "$dest"; then
                log_error "Failed to extract archive"
                return 1
            fi
            ;;
    esac
    
    log_success "Archive extracted"
}

# Find install directory
find_install_dir() {
    # Try ~/.local/bin first (user-wide, no sudo needed)
    if [ -d "$HOME/.local/bin" ]; then
        INSTALL_DIR="$HOME/.local/bin"
        SUDO=""
        return 0
    fi
    
    # Try /usr/local/bin (system-wide, may need sudo)
    if [ -w "/usr/local/bin" ]; then
        INSTALL_DIR="/usr/local/bin"
        SUDO=""
        return 0
    fi
    
    # If /usr/local/bin not writable, try with sudo
    if command -v sudo &> /dev/null; then
        INSTALL_DIR="/usr/local/bin"
        SUDO="sudo"
        return 0
    fi
    
    # Fallback to home directory
    INSTALL_DIR="$HOME/bin"
    SUDO=""
    
    if [ ! -d "$INSTALL_DIR" ]; then
        mkdir -p "$INSTALL_DIR"
    fi
}

# Install binary
install_binary() {
    local binary_path=$1
    
    log_info "Installing to $INSTALL_DIR..."
    
    if [ ! -f "$binary_path" ]; then
        log_error "Binary not found at $binary_path"
        return 1
    fi
    
    # Make executable
    chmod +x "$binary_path"
    
    # Copy to install directory
    if [ -n "$SUDO" ]; then
        if ! $SUDO cp "$binary_path" "$INSTALL_DIR/$BINARY_NAME"; then
            log_error "Failed to install binary"
            return 1
        fi
    else
        if ! cp "$binary_path" "$INSTALL_DIR/$BINARY_NAME"; then
            log_error "Failed to install binary"
            return 1
        fi
    fi
    
    log_success "Binary installed to $INSTALL_DIR/$BINARY_NAME"
}

# Verify installation
verify_installation() {
    log_info "Verifying installation..."
    
    if ! command -v "$BINARY_NAME" &> /dev/null; then
        log_warning "$BINARY_NAME not in PATH. You may need to add $INSTALL_DIR to your PATH."
        
        # Check if it exists at least
        if [ ! -f "$INSTALL_DIR/$BINARY_NAME" ]; then
            log_error "Installation failed"
            return 1
        fi
        
        # Try executing directly
        if ! "$INSTALL_DIR/$BINARY_NAME" --version &> /dev/null; then
            log_error "Binary exists but failed to execute"
            return 1
        fi
    else
        # Get version
        VERSION_OUTPUT=$("$BINARY_NAME" --version 2>/dev/null || echo "unknown")
        log_success "Installation successful!"
        log_info "Version: $VERSION_OUTPUT"
    fi
}

# Update PATH if needed
suggest_path_update() {
    if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
        log_warning "Add $INSTALL_DIR to your PATH to run 'runbeam' from anywhere"
        
        case "${SHELL##*/}" in
            zsh)
                log_info "Add to ~/.zshrc: export PATH=\"$INSTALL_DIR:\$PATH\""
                ;;
            bash)
                log_info "Add to ~/.bashrc: export PATH=\"$INSTALL_DIR:\$PATH\""
                ;;
            *)
                log_info "Add to your shell config: export PATH=\"$INSTALL_DIR:\$PATH\""
                ;;
        esac
    fi
}

# Cleanup
cleanup() {
    if [ -n "$TMP_DIR" ] && [ -d "$TMP_DIR" ]; then
        rm -rf "$TMP_DIR"
    fi
}

# Main installation flow
main() {
    log_info "Installing $BINARY_NAME from $REPO..."
    
    # Create temp directory
    TMP_DIR=$(mktemp -d)
    trap cleanup EXIT
    
    # Detect platform
    detect_platform
    
    # Get latest release
    get_latest_release
    
    # Construct URLs
    get_download_url
    
    # Download archive
    if ! download_file "$URL" "$TMP_DIR/$FILENAME"; then
        log_error "Installation failed"
        exit 1
    fi
    
    log_success "Download complete"
    
    # Verify checksum
    if ! verify_checksum "$TMP_DIR/$FILENAME" "$TMP_DIR/$CHECKSUM_FILE"; then
        log_error "Checksum verification failed. Installation aborted."
        exit 1
    fi
    
    # Extract archive
    if ! extract_archive "$TMP_DIR/$FILENAME" "$TMP_DIR"; then
        log_error "Installation failed"
        exit 1
    fi
    
    # Find binary in extracted files
    EXTRACTED_BINARY=$(find "$TMP_DIR" -name "$BINARY_NAME" -o -name "$BINARY_NAME.exe" 2>/dev/null | head -1)
    
    if [ -z "$EXTRACTED_BINARY" ]; then
        log_error "Could not find binary in extracted archive"
        exit 1
    fi
    
    # Find install directory
    find_install_dir
    
    # Install binary
    if ! install_binary "$EXTRACTED_BINARY"; then
        log_error "Installation failed"
        exit 1
    fi
    
    # Verify installation
    verify_installation
    
    # Suggest PATH update if needed
    suggest_path_update
    
    log_success "Installation complete!"
    log_info "Run '$BINARY_NAME --help' to get started"
}

# Run main function
main "$@"
