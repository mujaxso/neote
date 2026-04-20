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

echo "Downloading from: $DOWNLOAD_URL"
if curl -L -o "$FONT_DIR/$ZIP_FILE" "$DOWNLOAD_URL" --fail --progress-bar; then
    echo "✓ Downloaded JetBrainsMono.zip"
    
    # Check if the zip file is valid and not empty
    if [ ! -s "$FONT_DIR/$ZIP_FILE" ]; then
        echo "Error: Downloaded zip file is empty"
        rm -f "$FONT_DIR/$ZIP_FILE"
        exit 1
    fi
    
    # Create a temporary directory for extraction
    TEMP_DIR=$(mktemp -d)
    echo "Extracting font files..."
    
    # Extract the zip file
    if unzip -q "$FONT_DIR/$ZIP_FILE" -d "$TEMP_DIR" 2>/dev/null; then
        echo "✓ Extraction successful"
        
        # Find all .woff2 files in the extracted directory
        echo "Looking for .woff2 files..."
        find "$TEMP_DIR" -name "*.woff2" -type f | while read -r font_file; do
            filename=$(basename "$font_file")
            echo "  Found: $filename"
        done
        
        # Now, find and copy the specific files we need
        # The actual filenames in the zip might be different, so we need to be flexible
        
        # Look for Regular (not Italic)
        REGULAR_FILE=$(find "$TEMP_DIR" -name "*.woff2" -type f | grep -i "regular" | grep -v -i "italic" | head -1)
        if [ -n "$REGULAR_FILE" ]; then
            cp "$REGULAR_FILE" "$FONT_DIR/JetBrainsMonoNerdFont-Regular.woff2"
            echo "✓ Copied Regular variant"
        else
            echo "✗ Could not find Regular variant"
        fi
        
        # Look for Bold (not Italic)
        BOLD_FILE=$(find "$TEMP_DIR" -name "*.woff2" -type f | grep -i "bold" | grep -v -i "italic" | head -1)
        if [ -n "$BOLD_FILE" ]; then
            cp "$BOLD_FILE" "$FONT_DIR/JetBrainsMonoNerdFont-Bold.woff2"
            echo "✓ Copied Bold variant"
        else
            echo "✗ Could not find Bold variant"
        fi
        
        # Look for Italic (not Bold)
        ITALIC_FILE=$(find "$TEMP_DIR" -name "*.woff2" -type f | grep -i "italic" | grep -v -i "bold" | head -1)
        if [ -n "$ITALIC_FILE" ]; then
            cp "$ITALIC_FILE" "$FONT_DIR/JetBrainsMonoNerdFont-Italic.woff2"
            echo "✓ Copied Italic variant"
        else
            echo "✗ Could not find Italic variant"
        fi
        
        # Look for Bold Italic
        BOLD_ITALIC_FILE=$(find "$TEMP_DIR" -name "*.woff2" -type f | grep -i "bold" | grep -i "italic" | head -1)
        if [ -n "$BOLD_ITALIC_FILE" ]; then
            cp "$BOLD_ITALIC_FILE" "$FONT_DIR/JetBrainsMonoNerdFont-BoldItalic.woff2"
            echo "✓ Copied Bold Italic variant"
        else
            echo "✗ Could not find Bold Italic variant"
        fi
        
        # Clean up temporary directory
        rm -rf "$TEMP_DIR"
    else
        echo "Error: Failed to extract zip file"
        echo "The zip file might be corrupted or in an unexpected format"
        rm -f "$FONT_DIR/$ZIP_FILE"
        exit 1
    fi
    
    # Clean up the zip file
    rm -f "$FONT_DIR/$ZIP_FILE"
    
    # Verify we have the required files
    echo ""
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
        echo ""
        echo "✅ Success! All required font files are present."
        echo "Fonts are ready in: $FONT_DIR"
    else
        echo ""
        echo "⚠️  Warning: Some font files are missing."
        echo "Current contents of fonts directory:"
        ls -la "$FONT_DIR/" 2>/dev/null || echo "  (fonts directory is empty)"
        echo ""
        echo "You may need to manually download the fonts:"
        echo "1. Visit: https://www.nerdfonts.com/font-downloads"
        echo "2. Download 'JetBrainsMono.zip'"
        echo "3. Extract the following .woff2 files to $FONT_DIR:"
        echo "   - JetBrainsMonoNerdFont-Regular.woff2"
        echo "   - JetBrainsMonoNerdFont-Bold.woff2"
        echo "   - JetBrainsMonoNerdFont-Italic.woff2"
        echo "   - JetBrainsMonoNerdFont-BoldItalic.woff2"
    fi
    
else
    echo "Error: Failed to download JetBrainsMono.zip"
    echo "Possible reasons:"
    echo "1. Network connection issue"
    echo "2. The download URL may have changed"
    echo ""
    echo "You can manually download the font from:"
    echo "https://github.com/ryanoasis/nerd-fonts/releases"
    echo "Look for 'JetBrainsMono.zip' in the latest release"
    exit 1
fi

echo ""
echo "Font download process complete!"
