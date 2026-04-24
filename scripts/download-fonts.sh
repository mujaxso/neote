#!/usr/bin/env bash

# =============================================================================
# Professional Font Download Script for Zaroxi Studio
# Downloads and installs JetBrains Mono Nerd Font
# =============================================================================

set -euo pipefail

# -----------------------------------------------------------------------------
# Configuration
# -----------------------------------------------------------------------------
readonly SCRIPT_NAME=$(basename "$0")
readonly SCRIPT_VERSION="1.0.0"
readonly FONT_DIR="apps/desktop/frontend/public/fonts"
readonly NERD_FONTS_REPO="https://github.com/ryanoasis/nerd-fonts/releases/download/v3.4.0"
readonly ZIP_FILE="JetBrainsMono.zip"
readonly DOWNLOAD_URL="${NERD_FONTS_REPO}/${ZIP_FILE}"

# Font variants we need with their target filenames
declare -A FONT_TARGETS=(
    ["Regular"]="JetBrainsMonoNerdFont-Regular.ttf"
    ["Bold"]="JetBrainsMonoNerdFont-Bold.ttf"
    ["Italic"]="JetBrainsMonoNerdFont-Italic.ttf"
    ["BoldItalic"]="JetBrainsMonoNerdFont-BoldItalic.ttf"
)

# -----------------------------------------------------------------------------
# Logging functions
# -----------------------------------------------------------------------------
log_info() {
    echo "[INFO] $*"
}

log_success() {
    echo "✅ $*"
}

log_warning() {
    echo "⚠️  $*"
}

log_error() {
    echo "❌ $*" >&2
}

log_debug() {
    if [[ "${DEBUG:-false}" == "true" ]]; then
        echo "[DEBUG] $*"
    fi
}

# -----------------------------------------------------------------------------
# Utility functions
# -----------------------------------------------------------------------------
cleanup() {
    local exit_code=$?
    
    if [[ -n "${TEMP_DIR:-}" && -d "$TEMP_DIR" ]]; then
        log_debug "Removing temporary directory: $TEMP_DIR"
        rm -rf "$TEMP_DIR"
    fi
    
    if [[ -f "$FONT_DIR/$ZIP_FILE" ]]; then
        log_debug "Removing zip file: $FONT_DIR/$ZIP_FILE"
        rm -f "$FONT_DIR/$ZIP_FILE"
    fi
    
    if [[ $exit_code -eq 0 ]]; then
        log_success "Script completed successfully"
    else
        log_error "Script failed with exit code $exit_code"
    fi
}

print_usage() {
    cat << EOF
Usage: $SCRIPT_NAME [OPTIONS]

Downloads and installs JetBrains Mono Nerd Font for Zaroxi Studio.

Options:
    -h, --help      Show this help message
    -v, --version   Show version information
    -d, --debug     Enable debug output
    --clean         Clean the fonts directory before installation

Examples:
    $SCRIPT_NAME              # Download and install fonts
    $SCRIPT_NAME --clean      # Clean install
    $SCRIPT_NAME --debug      # Enable debug output

EOF
}

print_version() {
    echo "$SCRIPT_NAME version $SCRIPT_VERSION"
}

# -----------------------------------------------------------------------------
# Font discovery and installation
# -----------------------------------------------------------------------------
discover_font_file() {
    local variant="$1"
    local temp_dir="$2"
    
    case "$variant" in
        "Regular")
            # Look for files containing "Regular" but not "Italic"
            find "$temp_dir" -name "*.ttf" -type f | \
                grep -i "regular" | grep -v -i "italic" | head -1
            ;;
        "Bold")
            # Look for files containing "Bold" but not "Italic"
            find "$temp_dir" -name "*.ttf" -type f | \
                grep -i "bold" | grep -v -i "italic" | head -1
            ;;
        "Italic")
            # Look for files containing "Italic" but not "Bold"
            find "$temp_dir" -name "*.ttf" -type f | \
                grep -i "italic" | grep -v -i "bold" | head -1
            ;;
        "BoldItalic")
            # Look for files containing both "Bold" and "Italic"
            find "$temp_dir" -name "*.ttf" -type f | \
                grep -i "bold.*italic\|italic.*bold" | head -1
            ;;
        *)
            log_error "Unknown font variant: $variant"
            return 1
            ;;
    esac
}

install_fonts() {
    local temp_dir="$1"
    local clean="$2"
    
    log_info "Installing fonts to: $FONT_DIR"
    
    # Create fonts directory if it doesn't exist
    mkdir -p "$FONT_DIR"
    
    # Clean fonts directory if requested
    if [[ "$clean" == "true" ]]; then
        log_info "Cleaning fonts directory..."
        rm -f "$FONT_DIR"/*.ttf 2>/dev/null || true
    fi
    
    # Install each font variant
    local installed_count=0
    for variant in "${!FONT_TARGETS[@]}"; do
        local target_file="${FONT_TARGETS[$variant]}"
        local source_file=$(discover_font_file "$variant" "$temp_dir")
        
        if [[ -n "$source_file" && -f "$source_file" ]]; then
            cp "$source_file" "$FONT_DIR/$target_file"
            log_success "Installed $variant -> $target_file"
            ((installed_count++))
        else
            log_warning "Could not find source file for variant: $variant"
            log_debug "Searched for pattern: $variant"
        fi
    done
    
    echo "$installed_count"
}

verify_installation() {
    log_info "Verifying installation..."
    
    local all_present=true
    for variant in "${!FONT_TARGETS[@]}"; do
        local target_file="${FONT_TARGETS[$variant]}"
        local file_path="$FONT_DIR/$target_file"
        
        if [[ -f "$file_path" ]]; then
            local file_size=$(stat -f%z "$file_path" 2>/dev/null || stat -c%s "$file_path" 2>/dev/null || echo "0")
            if [[ $file_size -gt 1000 ]]; then
                log_success "$target_file ($((file_size/1024)) KB)"
            else
                log_warning "$target_file (file too small: ${file_size} bytes)"
                all_present=false
            fi
        else
            log_error "Missing: $target_file"
            all_present=false
        fi
    done
    
    if [[ "$all_present" == "true" ]]; then
        log_success "All required fonts are installed"
        return 0
    else
        log_warning "Some fonts are missing or invalid"
        return 1
    fi
}

# -----------------------------------------------------------------------------
# Main script execution
# -----------------------------------------------------------------------------
main() {
    local clean_install=false
    local debug_mode=false
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                print_usage
                exit 0
                ;;
            -v|--version)
                print_version
                exit 0
                ;;
            -d|--debug)
                debug_mode=true
                DEBUG=true
                shift
                ;;
            --clean)
                clean_install=true
                shift
                ;;
            *)
                log_error "Unknown option: $1"
                print_usage
                exit 1
                ;;
        esac
    done
    
    # Register cleanup handler
    trap cleanup EXIT
    
    log_info "Starting font installation..."
    log_debug "Font directory: $FONT_DIR"
    log_debug "Download URL: $DOWNLOAD_URL"
    
    # Create fonts directory
    mkdir -p "$FONT_DIR"
    
    # Download the font zip
    log_info "Downloading JetBrains Mono Nerd Font..."
    if ! curl -L -o "$FONT_DIR/$ZIP_FILE" "$DOWNLOAD_URL" --fail --progress-bar; then
        log_error "Failed to download font archive"
        log_info "Trying fallback to version 3.3.0..."
        local fallback_url="https://github.com/ryanoasis/nerd-fonts/releases/download/v3.3.0/$ZIP_FILE"
        if ! curl -L -o "$FONT_DIR/$ZIP_FILE" "$fallback_url" --fail --progress-bar; then
            log_error "Fallback download also failed"
            log_info "Please download manually from: https://github.com/ryanoasis/nerd-fonts/releases"
            exit 1
        fi
    fi
    
    # Verify downloaded file
    if [[ ! -s "$FONT_DIR/$ZIP_FILE" ]]; then
        log_error "Downloaded file is empty or corrupted"
        exit 1
    fi
    
    log_success "Download completed"
    
    # Create temporary directory for extraction
    TEMP_DIR=$(mktemp -d)
    log_debug "Created temporary directory: $TEMP_DIR"
    
    # Extract the zip file
    log_info "Extracting font files..."
    if ! unzip -q "$FONT_DIR/$ZIP_FILE" -d "$TEMP_DIR" 2>/dev/null; then
        log_error "Failed to extract font archive"
        exit 1
    fi
    
    log_success "Extraction completed"
    
    # List found font files (debug mode)
    if [[ "$debug_mode" == "true" ]]; then
        log_info "Found font files:"
        find "$TEMP_DIR" -name "*.ttf" -type f | head -10 | while read -r font_file; do
            echo "  - $(basename "$font_file")"
        done
    fi
    
    # Install fonts
    local installed_count=$(install_fonts "$TEMP_DIR" "$clean_install")
    
    # Verify installation
    if verify_installation; then
        log_success "Font installation completed successfully"
        log_info "Installed $installed_count font variants"
        log_info "Fonts are available at: $FONT_DIR"
        
        # Show directory contents
        log_info "Font directory contents:"
        ls -lh "$FONT_DIR"/*.ttf 2>/dev/null || log_warning "No font files found"
    else
        log_warning "Font installation completed with warnings"
        log_info "Some fonts may be missing. Check the output above for details."
        exit 1
    fi
}

# -----------------------------------------------------------------------------
# Script entry point
# -----------------------------------------------------------------------------
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
