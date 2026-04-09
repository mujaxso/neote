#!/usr/bin/env bash
set -e

# This script works in both bash and zsh environments
# Create fonts directory
mkdir -p apps/desktop/assets/fonts
cd apps/desktop/assets/fonts

# Download Nerd Fonts
echo "Downloading Nerd Fonts..."

# JetBrains Mono Nerd Font
curl -L -o "JetBrainsMonoNerdFont-Regular.ttf" \
    "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/JetBrainsMono/Ligatures/Regular/complete/JetBrains%20Mono%20Regular%20Nerd%20Font%20Complete.ttf"

# Fira Code Nerd Font
curl -L -o "FiraCodeNerdFont-Regular.ttf" \
    "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/FiraCode/Regular/complete/Fira%20Code%20Regular%20Nerd%20Font%20Complete.ttf"

# Cascadia Code Nerd Font
curl -L -o "CascadiaCodeNerdFont-Regular.ttf" \
    "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/CascadiaCode/Regular/complete/Caskaydia%20Cove%20Nerd%20Font%20Complete.ttf"

# Iosevka Nerd Font
curl -L -o "IosevkaNerdFont-Regular.ttf" \
    "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/Iosevka/Regular/complete/Iosevka%20Nerd%20Font%20Complete.ttf"

# Symbols Nerd Font (for icons)
curl -L -o "SymbolsNerdFont-Regular.ttf" \
    "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/NerdFontsSymbolsOnly/complete/Symbols-2048-em%20Nerd%20Font%20Complete.ttf"

# Noto Color Emoji
curl -L -o "NotoColorEmoji.ttf" \
    "https://github.com/googlefonts/noto-emoji/raw/main/fonts/NotoColorEmoji.ttf"

echo "Fonts downloaded successfully!"
#!/bin/bash

# Download fonts for Neote IDE
# This script downloads Nerd Fonts and other required fonts to the correct location

set -e

# Determine script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
FONT_DIR="$PROJECT_ROOT/apps/desktop/assets/fonts"

echo "Project root: $PROJECT_ROOT"
echo "Font directory: $FONT_DIR"

# Create font directory if it doesn't exist
mkdir -p "$FONT_DIR"

# Download Nerd Fonts symbols
echo "Downloading Symbols Nerd Font..."
SYMBOLS_URL="https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/NerdFontsSymbolsOnly.zip"
SYMBOLS_ZIP="$FONT_DIR/SymbolsNerdFont.zip"
curl -L "$SYMBOLS_URL" -o "$SYMBOLS_ZIP"

# Extract Symbols Nerd Font
echo "Extracting Symbols Nerd Font..."
unzip -q -o "$SYMBOLS_ZIP" -d "$FONT_DIR"
# Rename the extracted file to a consistent name
find "$FONT_DIR" -name "*.ttf" -type f | while read -r font; do
    if [[ "$font" == *"SymbolsOnly"* ]] || [[ "$font" == *"SymbolsNerdFont"* ]]; then
        mv "$font" "$FONT_DIR/SymbolsNerdFont-Regular.ttf"
    fi
done

# Download Noto Color Emoji
echo "Downloading Noto Color Emoji..."
NOTO_URL="https://github.com/googlefonts/noto-emoji/raw/main/fonts/NotoColorEmoji.ttf"
curl -L "$NOTO_URL" -o "$FONT_DIR/NotoColorEmoji.ttf"

# Download JetBrains Mono Nerd Font
echo "Downloading JetBrains Mono Nerd Font..."
JETBRAINS_URL="https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/JetBrainsMono.zip"
JETBRAINS_ZIP="$FONT_DIR/JetBrainsMono.zip"
curl -L "$JETBRAINS_URL" -o "$JETBRAINS_ZIP"

# Extract JetBrains Mono
echo "Extracting JetBrains Mono Nerd Font..."
unzip -q -o "$JETBRAINS_ZIP" -d "$FONT_DIR"
# Find and rename the regular variant
find "$FONT_DIR" -name "*JetBrainsMonoNerdFont-Regular.ttf" -type f | head -1 | while read -r font; do
    mv "$font" "$FONT_DIR/JetBrainsMonoNerdFont-Regular.ttf"
done

# Download Fira Code Nerd Font
echo "Downloading Fira Code Nerd Font..."
FIRACODE_URL="https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/FiraCode.zip"
FIRACODE_ZIP="$FONT_DIR/FiraCode.zip"
curl -L "$FIRACODE_URL" -o "$FIRACODE_ZIP"

# Extract Fira Code
echo "Extracting Fira Code Nerd Font..."
unzip -q -o "$FIRACODE_ZIP" -d "$FONT_DIR"
find "$FONT_DIR" -name "*FiraCodeNerdFont-Regular.ttf" -type f | head -1 | while read -r font; do
    mv "$font" "$FONT_DIR/FiraCodeNerdFont-Regular.ttf"
done

# Download Cascadia Code Nerd Font
echo "Downloading Cascadia Code Nerd Font..."
CASCADIA_URL="https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/CascadiaCode.zip"
CASCADIA_ZIP="$FONT_DIR/CascadiaCode.zip"
curl -L "$CASCADIA_URL" -o "$CASCADIA_ZIP"

# Extract Cascadia Code
echo "Extracting Cascadia Code Nerd Font..."
unzip -q -o "$CASCADIA_ZIP" -d "$FONT_DIR"
find "$FONT_DIR" -name "*CascadiaCodeNerdFont-Regular.ttf" -type f | head -1 | while read -r font; do
    mv "$font" "$FONT_DIR/CascadiaCodeNerdFont-Regular.ttf"
done

# Download Iosevka Nerd Font
echo "Downloading Iosevka Nerd Font..."
IOSEVKA_URL="https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/Iosevka.zip"
IOSEVKA_ZIP="$FONT_DIR/Iosevka.zip"
curl -L "$IOSEVKA_URL" -o "$IOSEVKA_ZIP"

# Extract Iosevka
echo "Extracting Iosevka Nerd Font..."
unzip -q -o "$IOSEVKA_ZIP" -d "$FONT_DIR"
find "$FONT_DIR" -name "*IosevkaNerdFont-Regular.ttf" -type f | head -1 | while read -r font; do
    mv "$font" "$FONT_DIR/IosevkaNerdFont-Regular.ttf"
done

# Download regular JetBrains Mono (non-Nerd Font)
echo "Downloading JetBrains Mono regular..."
JETBRAINS_REGULAR_URL="https://github.com/JetBrains/JetBrainsMono/releases/download/v2.304/JetBrainsMono-2.304.zip"
JETBRAINS_REGULAR_ZIP="$FONT_DIR/JetBrainsMonoRegular.zip"
curl -L "$JETBRAINS_REGULAR_URL" -o "$JETBRAINS_REGULAR_ZIP"

# Extract regular JetBrains Mono
echo "Extracting JetBrains Mono regular..."
unzip -q -o "$JETBRAINS_REGULAR_ZIP" -d "$FONT_DIR"
find "$FONT_DIR" -name "JetBrainsMono-Regular.ttf" -type f | head -1 | while read -r font; do
    mv "$font" "$FONT_DIR/JetBrainsMono-Regular.ttf"
done

# Download regular Fira Code (non-Nerd Font)
echo "Downloading Fira Code regular..."
FIRACODE_REGULAR_URL="https://github.com/tonsky/FiraCode/releases/download/6.2/Fira_Code_v6.2.zip"
FIRACODE_REGULAR_ZIP="$FONT_DIR/FiraCodeRegular.zip"
curl -L "$FIRACODE_REGULAR_URL" -o "$FIRACODE_REGULAR_ZIP"

# Extract regular Fira Code
echo "Extracting Fira Code regular..."
unzip -q -o "$FIRACODE_REGULAR_ZIP" -d "$FONT_DIR"
find "$FONT_DIR" -name "FiraCode-Regular.ttf" -type f | head -1 | while read -r font; do
    mv "$font" "$FONT_DIR/FiraCode-Regular.ttf"
done

# Clean up zip files
echo "Cleaning up temporary files..."
rm -f "$FONT_DIR"/*.zip
rm -f "$FONT_DIR"/*.txt
rm -f "$FONT_DIR"/*.md
rm -f "$FONT_DIR"/*.html

# List downloaded fonts
echo "Downloaded fonts:"
ls -la "$FONT_DIR"/*.ttf

echo "Font download complete!"
