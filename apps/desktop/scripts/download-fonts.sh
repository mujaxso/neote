#!/usr/bin/env bash
set -euo pipefail

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Go up one level to the desktop directory
DESKTOP_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
FONTS_DIR="${DESKTOP_DIR}/assets/fonts"

echo "Downloading programming fonts to ${FONTS_DIR}..."
mkdir -p "${FONTS_DIR}"

# Remove old font files if they exist
rm -f "${FONTS_DIR}/"*.ttf

# Download JetBrains Mono (Regular)
JETBRAINS_MONO_URL="https://github.com/JetBrains/JetBrainsMono/raw/master/fonts/ttf/JetBrainsMono-Regular.ttf"

# Download Fira Code (Regular) as alternative
FIRACODE_URL="https://github.com/tonsky/FiraCode/raw/main/distr/ttf/FiraCode-Regular.ttf"

# Download a fallback emoji font (still needed for icons)
EMOJI_URL="https://github.com/googlefonts/noto-emoji/raw/main/fonts/NotoColorEmoji.ttf"

echo "Downloading JetBrains Mono..."
if command -v curl &> /dev/null; then
    if curl -L -o "${FONTS_DIR}/JetBrainsMono-Regular.ttf" "${JETBRAINS_MONO_URL}"; then
        echo "✓ JetBrains Mono downloaded successfully"
    else
        echo "✗ Failed to download JetBrains Mono"
        exit 1
    fi
elif command -v wget &> /dev/null; then
    if wget -O "${FONTS_DIR}/JetBrainsMono-Regular.ttf" "${JETBRAINS_MONO_URL}"; then
        echo "✓ JetBrains Mono downloaded successfully"
    else
        echo "✗ Failed to download JetBrains Mono"
        exit 1
    fi
else
    echo "Error: Neither curl nor wget found. Please install one of them."
    exit 1
fi

echo "Downloading Fira Code..."
if command -v curl &> /dev/null; then
    if curl -L -o "${FONTS_DIR}/FiraCode-Regular.ttf" "${FIRACODE_URL}"; then
        echo "✓ Fira Code downloaded successfully"
    else
        echo "✗ Failed to download Fira Code"
        exit 1
    fi
elif command -v wget &> /dev/null; then
    if wget -O "${FONTS_DIR}/FiraCode-Regular.ttf" "${FIRACODE_URL}"; then
        echo "✓ Fira Code downloaded successfully"
    else
        echo "✗ Failed to download Fira Code"
        exit 1
    fi
else
    echo "Error: Neither curl nor wget found. Please install one of them."
    exit 1
fi

echo "Downloading Noto Color Emoji for icons..."
if command -v curl &> /dev/null; then
    if curl -L -o "${FONTS_DIR}/NotoColorEmoji.ttf" "${EMOJI_URL}"; then
        echo "✓ Noto Color Emoji downloaded successfully"
    else
        echo "✗ Failed to download Noto Color Emoji"
        exit 1
    fi
elif command -v wget &> /dev/null; then
    if wget -O "${FONTS_DIR}/NotoColorEmoji.ttf" "${EMOJI_URL}"; then
        echo "✓ Noto Color Emoji downloaded successfully"
    else
        echo "✗ Failed to download Noto Color Emoji"
        exit 1
    fi
else
    echo "Error: Neither curl nor wget found. Please install one of them."
    exit 1
fi

# Verify the downloads
if [[ -f "${FONTS_DIR}/JetBrainsMono-Regular.ttf" ]] && \
   [[ -f "${FONTS_DIR}/FiraCode-Regular.ttf" ]] && \
   [[ -f "${FONTS_DIR}/NotoColorEmoji.ttf" ]]; then
    echo ""
    echo "✅ Fonts downloaded successfully!"
    echo "   JetBrains Mono: ${FONTS_DIR}/JetBrainsMono-Regular.ttf"
    echo "   Fira Code: ${FONTS_DIR}/FiraCode-Regular.ttf"
    echo "   Noto Color Emoji: ${FONTS_DIR}/NotoColorEmoji.ttf"
    echo ""
    echo "To rebuild the application with the new fonts:"
    echo "  cd ${DESKTOP_DIR}"
    echo "  cargo build --bin desktop"
else
    echo "❌ Error: Some font files failed to download."
    exit 1
fi
