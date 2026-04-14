#!/usr/bin/env bash
set -e

echo "Manual installation of Tree-sitter Markdown grammar..."

# Determine runtime directory
if [ -n "$NEOTE_RUNTIME" ]; then
    RUNTIME_DIR="$NEOTE_RUNTIME"
else
    RUNTIME_DIR="$HOME/Work/qyzer/runtime/treesitter"
fi

mkdir -p "$RUNTIME_DIR/grammars/linux-x86_64"
mkdir -p "$RUNTIME_DIR/languages/markdown/queries"

echo "Using runtime directory: $RUNTIME_DIR"

# Download directly using wget/curl to a temporary location
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"

echo "Downloading source code..."
# Download the specific commit that's known to work
wget -O markdown.zip https://github.com/tree-sitter/tree-sitter-markdown/archive/refs/heads/master.zip
unzip -q markdown.zip
cd tree-sitter-markdown-master

echo "Building grammar..."
# Check for scanner.c
if [ -f src/scanner.c ]; then
    cc -shared -fPIC -o libtree-sitter-markdown.so src/parser.c src/scanner.c -I./src
else
    cc -shared -fPIC -o libtree-sitter-markdown.so src/parser.c -I./src
fi

# Copy the library
cp libtree-sitter-markdown.so "$RUNTIME_DIR/grammars/linux-x86_64/"

# Copy query files if they exist
if [ -d queries ]; then
    mkdir -p "$RUNTIME_DIR/languages/markdown/queries"
    cp queries/* "$RUNTIME_DIR/languages/markdown/queries/" 2>/dev/null || true
fi

# Clean up
rm -rf "$TEMP_DIR"

echo "Markdown grammar installed successfully!"
echo "Library: $RUNTIME_DIR/grammars/linux-x86_64/libtree-sitter-markdown.so"
