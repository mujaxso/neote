#!/usr/bin/env bash

# Script to download JetBrains Mono Nerd Font for Zaroxi Studio
# Run this script from the project root directory

set -euo pipefail

# Configuration
FONT_DIR="apps/desktop/frontend/public/fonts"
NERD_FONTS_REPO="https://github.com/ryanoasis/nerd-fonts/releases/download/v3.3.0"

# Create fonts directory if it doesn't exist
mkdir -p "$FONT_DIR"

echo "Downloading JetBrains Mono Nerd Font to $FONT_DIR..."

# Download the complete font zip
ZIP_FILE="JetBrainsMono.zip"
DOWNLOAD_URL="${NERD_FONTS_REPO}/${ZIP_FILE}"

if curl -L -o "$FONT_DIR/$ZIP_FILE" "$DOWNLOAD_URL" --fail --silent --show-error; then
    echo "Downloaded JetBrainsMono.zip"
    
    # Extract only the specific .woff2 files we need
    echo "Extracting required font files..."
    
    # List of files we need to extract
    FONT_FILES=(
        "JetBrains Mono Regular Nerd Font Complete Mono.woff2"
        "JetBrains Mono Bold Nerd Font Complete Mono.woff2"
        "JetBrains Mono Italic Nerd Font Complete Mono.woff2"
        "JetBrains Mono Bold Italic Nerd Font Complete Mono.woff2"
    )
    
    # Extract each file and rename to match our CSS @font-face declarations
    for font_file in "${FONT_FILES[@]}"; do
        if unzip -q -j "$FONT_DIR/$ZIP_FILE" "$font_file" -d "$FONT_DIR" 2>/dev/null; then
            echo "  Extracted: $font_file"
        else
            # Try alternative naming pattern
            alt_file="${font_file// Complete Mono/}"
            if unzip -q -j "$FONT_DIR/$ZIP_FILE" "$alt_file" -d "$FONT_DIR" 2>/dev/null; then
                echo "  Extracted (alternative): $alt_file"
            else
                echo "  Warning: Could not extract $font_file"
            fi
        fi
    done
    
    # Rename files to match our CSS @font-face declarations
    echo "Renaming files to match CSS @font-face declarations..."
    
    # Map original filenames to our desired filenames
    declare -A RENAME_MAP=(
        ["JetBrains Mono Regular Nerd Font Complete Mono.woff2"]="JetBrainsMonoNerdFont-Regular.woff2"
        ["JetBrains Mono Bold Nerd Font Complete Mono.woff2"]="JetBrainsMonoNerdFont-Bold.woff2"
        ["JetBrains Mono Italic Nerd Font Complete Mono.woff2"]="JetBrainsMonoNerdFont-Italic.woff2"
        ["JetBrains Mono Bold Italic Nerd Font Complete Mono.woff2"]="JetBrainsMonoNerdFont-BoldItalic.woff2"
        ["JetBrains Mono Regular Nerd Font.woff2"]="JetBrainsMonoNerdFont-Regular.woff2"
        ["JetBrains Mono Bold Nerd Font.woff2"]="JetBrainsMonoNerdFont-Bold.woff2"
        ["JetBrains Mono Italic Nerd Font.woff2"]="JetBrainsMonoNerdFont-Italic.woff2"
        ["JetBrains Mono Bold Italic Nerd Font.woff2"]="JetBrainsMonoNerdFont-BoldItalic.woff2"
    )
    
    for old_name in "${!RENAME_MAP[@]}"; do
        if [ -f "$FONT_DIR/$old_name" ]; then
            new_name="${RENAME_MAP[$old_name]}"
            mv "$FONT_DIR/$old_name" "$FONT_DIR/$new_name"
            echo "  Renamed: $old_name -> $new_name"
        fi
    done
    
    # Clean up extracted zip and any leftover files
    rm -f "$FONT_DIR/$ZIP_FILE"
    rm -f "$FONT_DIR/"*.txt 2>/dev/null || true
    rm -f "$FONT_DIR/"*.md 2>/dev/null || true
    
    echo "Cleaned up temporary files"
    
    # Verify we have the required files
    echo "Verifying downloaded fonts..."
    REQUIRED_FILES=(
        "JetBrainsMonoNerdFont-Regular.woff2"
        "JetBrainsMonoNerdFont-Bold.woff2"
        "JetBrainsMonoNerdFont-Italic.woff2"
        "JetBrainsMonoNerdFont-BoldItalic.woff2"
    )
    
    all_present=true
    for required_file in "${REQUIRED_FILES[@]}"; do
        if [ -f "$FONT_DIR/$required_file" ]; then
            echo "  ✓ $required_file"
        else
            echo "  ✗ Missing: $required_file"
            all_present=false
        fi
    done
    
    if [ "$all_present" = true ]; then
        echo "Success! All required font files are present."
    else
        echo "Warning: Some font files are missing. You may need to download them manually."
        echo "Visit: https://www.nerdfonts.com/font-downloads"
    fi
    
else
    echo "Error: Failed to download JetBrainsMono.zip"
    echo "You can manually download the font from:"
    echo "https://github.com/ryanoasis/nerd-fonts/releases"
    echo "Or visit: https://www.nerdfonts.com/font-downloads"
    exit 1
fi

echo "Font download complete!"
