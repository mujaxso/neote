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
    
    # First, list the contents to see what's actually in the zip
    echo "Checking zip contents..."
    unzip -l "$FONT_DIR/$ZIP_FILE" | grep -i "\.woff2" | head -20
    
    # Extract all .woff2 files to a temporary directory
    TEMP_DIR=$(mktemp -d)
    echo "Extracting all .woff2 files to temporary directory..."
    
    # Extract all .woff2 files
    if unzip -q -j "$FONT_DIR/$ZIP_FILE" "*.woff2" -d "$TEMP_DIR" 2>/dev/null; then
        echo "Extracted .woff2 files successfully"
        
        # Find the specific files we need
        # The actual filenames might vary, so we'll look for patterns
        find "$TEMP_DIR" -name "*.woff2" -type f | while read -r font_file; do
            filename=$(basename "$font_file")
            
            # Determine which variant this is based on the filename
            case "$filename" in
                *"Regular"*)
                    if [[ "$filename" == *"Italic"* ]]; then
                        # This should be Italic, not Regular
                        continue
                    fi
                    target_name="JetBrainsMonoNerdFont-Regular.woff2"
                    ;;
                *"Bold"*)
                    if [[ "$filename" == *"Italic"* ]]; then
                        target_name="JetBrainsMonoNerdFont-BoldItalic.woff2"
                    else
                        target_name="JetBrainsMonoNerdFont-Bold.woff2"
                    fi
                    ;;
                *"Italic"*)
                    if [[ "$filename" != *"Bold"* ]]; then
                        target_name="JetBrainsMonoNerdFont-Italic.woff2"
                    else
                        # Already handled in Bold* case
                        continue
                    fi
                    ;;
                *)
                    # Skip files that don't match our patterns
                    continue
                    ;;
            esac
            
            # Copy to the font directory with the correct name
            cp "$font_file" "$FONT_DIR/$target_name"
            echo "  Copied: $filename -> $target_name"
        done
        
        # Clean up temporary directory
        rm -rf "$TEMP_DIR"
    else
        echo "Error: Failed to extract .woff2 files from the zip"
        echo "Trying alternative extraction method..."
        
        # Try extracting everything and then filtering
        TEMP_DIR2=$(mktemp -d)
        unzip -q "$FONT_DIR/$ZIP_FILE" -d "$TEMP_DIR2" 2>/dev/null || true
        
        # Find .woff2 files in the extracted directory
        find "$TEMP_DIR2" -name "*.woff2" -type f | while read -r font_file; do
            filename=$(basename "$font_file")
            
            # Use a simpler approach: look for specific patterns
            if [[ "$filename" == *"Regular"* ]] && [[ "$filename" != *"Italic"* ]]; then
                cp "$font_file" "$FONT_DIR/JetBrainsMonoNerdFont-Regular.woff2"
                echo "  Found Regular variant"
            elif [[ "$filename" == *"Bold"* ]] && [[ "$filename" != *"Italic"* ]]; then
                cp "$font_file" "$FONT_DIR/JetBrainsMonoNerdFont-Bold.woff2"
                echo "  Found Bold variant"
            elif [[ "$filename" == *"Italic"* ]] && [[ "$filename" != *"Bold"* ]]; then
                cp "$font_file" "$FONT_DIR/JetBrainsMonoNerdFont-Italic.woff2"
                echo "  Found Italic variant"
            elif [[ "$filename" == *"Bold"* ]] && [[ "$filename" == *"Italic"* ]]; then
                cp "$font_file" "$FONT_DIR/JetBrainsMonoNerdFont-BoldItalic.woff2"
                echo "  Found Bold Italic variant"
            fi
        done
        
        rm -rf "$TEMP_DIR2"
    fi
    
    # Clean up the zip file
    rm -f "$FONT_DIR/$ZIP_FILE"
    
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
        echo "Warning: Some font files are missing."
        echo "The zip file structure may have changed. Here's what's in the fonts directory:"
        ls -la "$FONT_DIR/" 2>/dev/null || echo "  (fonts directory is empty)"
        echo ""
        echo "You can manually download the font from:"
        echo "https://github.com/ryanoasis/nerd-fonts/releases"
        echo "Or visit: https://www.nerdfonts.com/font-downloads"
        echo ""
        echo "Alternatively, you can try downloading individual files from:"
        echo "https://github.com/ryanoasis/nerd-fonts/tree/master/patched-fonts/JetBrainsMono"
    fi
    
else
    echo "Error: Failed to download JetBrainsMono.zip"
    echo "You can manually download the font from:"
    echo "https://github.com/ryanoasis/nerd-fonts/releases"
    echo "Or visit: https://www.nerdfonts.com/font-downloads"
    exit 1
fi

echo "Font download complete!"
