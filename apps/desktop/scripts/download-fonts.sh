#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
FONTS_DIR="${PROJECT_ROOT}/assets/fonts"

echo "Downloading fonts to ${FONTS_DIR}..."
mkdir -p "${FONTS_DIR}"

# Remove placeholder files if they exist
rm -f "${FONTS_DIR}/NotoSans-Regular.ttf"
rm -f "${FONTS_DIR}/NotoEmoji-Regular.ttf"

# Download Noto Sans Regular from a reliable source
# Using the official Google Fonts GitHub repository
NOTO_SANS_URL="https://github.com/googlefonts/noto-fonts/raw/main/hinted/ttf/NotoSans/NotoSans-Regular.ttf"

# Download Noto Color Emoji (which includes emoji support)
# Note: NotoEmoji-Regular.ttf might not exist, so we'll use NotoColorEmoji.ttf
NOTO_EMOJI_URL="https://github.com/googlefonts/noto-emoji/raw/main/fonts/NotoColorEmoji.ttf"

echo "Downloading Noto Sans Regular..."
if command -v curl &> /dev/null; then
    if curl -L -o "${FONTS_DIR}/NotoSans-Regular.ttf" "${NOTO_SANS_URL}"; then
        echo "✓ Noto Sans downloaded successfully"
    else
        echo "✗ Failed to download Noto Sans"
        exit 1
    fi
elif command -v wget &> /dev/null; then
    if wget -O "${FONTS_DIR}/NotoSans-Regular.ttf" "${NOTO_SANS_URL}"; then
        echo "✓ Noto Sans downloaded successfully"
    else
        echo "✗ Failed to download Noto Sans"
        exit 1
    fi
else
    echo "Error: Neither curl nor wget found. Please install one of them."
    exit 1
fi

echo "Downloading Noto Color Emoji..."
if command -v curl &> /dev/null; then
    if curl -L -o "${FONTS_DIR}/NotoColorEmoji.ttf" "${NOTO_EMOJI_URL}"; then
        echo "✓ Noto Color Emoji downloaded successfully"
        # Also create a symlink for the expected name
        ln -sf "NotoColorEmoji.ttf" "${FONTS_DIR}/NotoEmoji-Regular.ttf"
    else
        echo "✗ Failed to download Noto Color Emoji"
        exit 1
    fi
elif command -v wget &> /dev/null; then
    if wget -O "${FONTS_DIR}/NotoColorEmoji.ttf" "${NOTO_EMOJI_URL}"; then
        echo "✓ Noto Color Emoji downloaded successfully"
        # Also create a symlink for the expected name
        ln -sf "NotoColorEmoji.ttf" "${FONTS_DIR}/NotoEmoji-Regular.ttf"
    else
        echo "✗ Failed to download Noto Color Emoji"
        exit 1
    fi
else
    echo "Error: Neither curl nor wget found. Please install one of them."
    exit 1
fi

# Verify the downloads
if [[ -f "${FONTS_DIR}/NotoSans-Regular.ttf" ]] && [[ -f "${FONTS_DIR}/NotoColorEmoji.ttf" ]]; then
    echo ""
    echo "✅ Fonts downloaded successfully!"
    echo "   Noto Sans Regular: ${FONTS_DIR}/NotoSans-Regular.ttf"
    echo "   Noto Color Emoji: ${FONTS_DIR}/NotoColorEmoji.ttf"
    echo ""
    echo "You may need to rebuild the application for the fonts to take effect:"
    echo "  cargo build --bin desktop"
else
    echo "❌ Error: Some font files failed to download."
    exit 1
fi
